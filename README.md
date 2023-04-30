![logo](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/1417c5138afef6d1cc7b5086ab008774a8f35f20/logo)
# IDS721 Final Project: News Sentiment Analysis MLOps Microservice with Rust and Python
Are you curious about the public sentiment surrounding a specific topic? For instance:
- What is the public sentiment towards the latest news on the US election?
- What is the public sentiment towards the latest news on the COVID-19 pandemic?
- If you are engaged in stock trading, you might be interested in the public sentiment regarding a particular stock.

This repository contains a project that performs sentiment analysis on latest news using Rust and Python as MLOps microservices.
## Overview
The project consists of the following components to predict the public sentiment of given keywords:
- a pre-trained [**RoBERTa**](https://huggingface.co/docs/transformers/model_doc/roberta) model 
- information retrieved from [**DuckDuckGo**](https://serpapi.com/duckduckgo-news-results)  
- async server is built upon [**Rust tokio**](https://docs.rs/tokio/latest/tokio/)

## Demo
First, we input keyword "Apple"

![Apple](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/edde10a57ccb9420876e597d44463ca71d26749b/Apple.png)

Try another keyword "Meta"

![Meta](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/edde10a57ccb9420876e597d44463ca71d26749b/Meta.png)

We can also perform sentiment analysis on a specific person, for example "Elon Mask"

![Elon Mask](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/edde10a57ccb9420876e597d44463ca71d26749b/Elon%20Mask.png)

## Prerequisites
To use this project, you need to have the following:
- Rust and Cargo installed on your local machine
- Python 3 installed on your local machine

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
## Usage
### Run on AWS
You can access my project directly through [this link](http://3.90.27.105:8080/)
![AWS](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/edde10a57ccb9420876e597d44463ca71d26749b/AWS.png)

### Run Locally

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

## Load Test

## Notes
In the first run, the model will be downloaded and cached in the `~/.cache/huggingface/transformers` folder. *This will take a while depending on your network speed*.

*Check [design.md](docs/design.md) for more details on the overall design struction of this project.*
## Reference
