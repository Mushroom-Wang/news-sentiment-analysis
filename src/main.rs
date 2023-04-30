use std::net::SocketAddr;

mod web_server;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    web_server::run(addr).await;
}
