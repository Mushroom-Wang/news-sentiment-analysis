![logo](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/1417c5138afef6d1cc7b5086ab008774a8f35f20/logo)

Do you want to know public sentiment about a particular topic? For example,
- what is the public sentiment about the latest news on the US election;
- or what is the public sentiment about the latest news on the COVID-19 pandemic?
- If you are trading stocks, you may also want to know the public sentiment about a particular stock.

This is a web app that uses 
- a pre-trained **RoBERTa** model 
- and information retrieved from **DuckDuckGo**  

to predict the public sentiment of given keywords. 

Its async server is built upon **Rust tokio**. 

In short, yet just another sentiment analysis tool for the latest news.

## Demo 
DEMO HERE

## Installation
1. Create a python virtual environment (or whatever you prefer)
```bash
python3 -m venv venv
source venv/bin/activate
```

2. Install python dependencies
```bash
cd sentiment_analysis/
pip install -e .
```


## Run on AWS
DETAILS HERE

## Run Locally

1. Start the API server

```bash
python -m roberta_sentiment.start  
```

This will take one terminal. To run in the background, use `nohup` or `screen`.

2. Start the async Rust server

```bash
cargo run 
```

This will start an async rust server on port 8080.

3. Access the website by going to `localhost:8080`. Ready to go!

## Notes
In the first run, the model will be downloaded and cached in the `~/.cache/huggingface/transformers` folder. *This will take a while depending on your network speed*.

*Check [design.md](docs/design.md) for more details on the overall design struction of this project.*
