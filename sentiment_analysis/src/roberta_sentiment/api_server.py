import json
import urllib.parse
from http.server import BaseHTTPRequestHandler
from socketserver import TCPServer

from roberta_sentiment.ddg_search import DDGNewsSearch
from roberta_sentiment.sentiment import SentimentAnalysis

DDG = DDGNewsSearch()
SENTIMENT = SentimentAnalysis()


class APIHandler(BaseHTTPRequestHandler):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)

    def do_GET(self):
        parsed_path = urllib.parse.urlparse(self.path)
        query_params = urllib.parse.parse_qs(parsed_path.query)

        if parsed_path.path == "/api" and "q" in query_params:
            search_query = query_params["q"][0]
            news = DDG.get_results(search_query)
            pred_res = SENTIMENT.predict([n["body"] for n in news])

            for n, pred in zip(news, pred_res):
                n["sentiment"] = pred["label"]
                n["confidence"] = pred["score"]

            response_text = json.dumps(news)

            self.send_response(200)
            self.send_header("Content-type", "text/html")
            self.end_headers()
            self.wfile.write(response_text.encode("utf-8"))


def start_server():
    print("Starting server")
    server = TCPServer(("localhost", 8000), APIHandler, bind_and_activate=False)
    server.timeout = 60
    server.allow_reuse_address = True
    server.server_bind()
    server.server_activate()
    print("Serving at port 8000")

    server.serve_forever()
