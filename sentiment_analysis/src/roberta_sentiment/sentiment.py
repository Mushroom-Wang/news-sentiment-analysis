from transformers import pipeline


class SentimentAnalysis:
    def __init__(self):
        self.sent_rob = pipeline(
            "sentiment-analysis", model="siebert/sentiment-roberta-large-english"
        )

    def predict(self, text: str):
        return self.sent_rob(text)
