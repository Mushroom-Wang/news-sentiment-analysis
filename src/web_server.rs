use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use lazy_static::lazy_static;
use reqwest::Client;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

const ROOT_PATH: &str = env!("CARGO_MANIFEST_DIR");
const API_URL: &str = "http://localhost:8000/api?q=";

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/search") => {
            let query = req.uri().query().unwrap_or("");
            let term = query
                .split('&')
                .filter_map(|param| {
                    let mut split = param.splitn(2, '=');
                    if split.next()? == "term" {
                        Some(split.next()?.to_string())
                    } else {
                        None
                    }
                })
                .next()
                .unwrap_or_default();

            let url = API_URL.to_owned() + &term;
            let resp = CLIENT.get(url).send().await.unwrap();
            let body = resp.text().await.unwrap();

            let response = Response::builder()
                .header("Content-Type", "application/json")
                .body(Body::from(body))
                .unwrap();

            Ok(response)
        }
        _ => serve_static_file(req).await,
    }
}

async fn serve_static_file(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut path = PathBuf::from(ROOT_PATH);
    path.push("pages");
    let req_path = req.uri().path();
    if req_path == "/" {
        path.push("index.html");
    } else {
        path.push(req_path.trim_start_matches('/'));
    }

    let mut file = match File::open(&path).await {
        Ok(file) => file,
        Err(_) => {
            return Ok(Response::builder()
                .status(404)
                .body("File not found".into())
                .unwrap())
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents).await {
        Ok(_) => Ok(Response::new(contents.into())),
        Err(_) => Ok(Response::builder()
            .status(500)
            .body("Internal server error".into())
            .unwrap()),
    }
}

pub async fn run(addr: SocketAddr) {
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
