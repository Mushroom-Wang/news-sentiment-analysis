A simple API server for
- [roberta-base-sentiment-analysis](https://huggingface.co/cardiffnlp/twitter-roberta-base-sentiment) model from huggingface
- Duckduckgo news search engine

## Installation

```bash
pip install -e .
```

## Run

Start the API server

```bash
python -m roberta_sentiment.start  
```

This will take on terminal. To run in background, use `nohup` or `screen`.


## Notes
In the first run, the model will be downloaded and cached in the `~/.cache/huggingface/transformers` folder. This will take a while.