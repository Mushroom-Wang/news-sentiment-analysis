![logo](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/1417c5138afef6d1cc7b5086ab008774a8f35f20/logo)
# IDS721 Final Project: News Sentiment Analysis MLOps Microservice with Rust and Python
Are you curious about the public sentiment surrounding a specific topic? For instance:
- What is the public sentiment towards the latest news on the US election?
- What is the public sentiment towards the latest news on the COVID-19 pandemic?
- If you are engaged in stock trading, you might be interested in the public sentiment regarding a particular stock.

This repository contains a project that performs sentiment analysis on latest news using Rust and Python as MLOps microservices.
## Overview
The is a one-person final project consists of the following components to predict the public sentiment of given keywords:
- a pre-trained [**RoBERTa**](https://huggingface.co/docs/transformers/model_doc/roberta) model 
- information retrieved from [**DuckDuckGo**](https://serpapi.com/duckduckgo-news-results)  
- async server is built upon [**Rust tokio**](https://docs.rs/tokio/latest/tokio/)
![Project Structure](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/09c4f63f1a750e04e82b980e0579e0f0359cbd1e/images/Project%20Structure)
*Check [design.md](docs/design.md) for more details on the overall design struction of this project.*

## Demo
You can access my project directly through [this link](http://3.90.27.105:8080/). I also recorded a [demo video]() posted on Youtube.

First, we input keyword "Apple" and get the following news and their sentiments.

![Apple](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/edde10a57ccb9420876e597d44463ca71d26749b/Apple.png)

Try another keyword "Meta":

![Meta](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/edde10a57ccb9420876e597d44463ca71d26749b/Meta.png)

We can also perform sentiment analysis on a specific person, for example "Elon Mask".

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
I deployed the project to AWS EC2 so that public can access my project directly through [this link](http://3.90.27.105:8080/).
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
This will start an async rust server on port 8080. In the first run, the model will be downloaded and cached in the `~/.cache/huggingface/transformers` folder. *This will take a while depending on your network speed*.

3. Access the website by going to `localhost:8080`. Ready to use!

## Load Testing
Load testing is the process of evaluating a system's performance under high load conditions. It is essential to ensure that a system can handle the expected load and traffic without slowing down or crashing. I use [WebPageTest](https://www.webpagetest.org/) to conduct the load testing. Check out these web performance test results on [WebPageTest.org](https://www.webpagetest.org/result/230430_BiDcY6_67W/). The overall perfermance is not bad:
- This site was quick to connect and deliver initial code. It began rendering content very quickly. There were 1 render-blocking requests. The largest content rendered quickly.
- This site had good layout stability. It took little time to become interactive. It had 2 accessibility issues, none serious. HTML content was mostly generated server-side.
- This site had no render-blocking 3rd party requests that could be a single point of failure. It had no security issues. HTML content was mostly generated server-side.

![Load Testing1](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/77f22b63c53ce749c4c617c156740e70d139b83d/Load%20Testing1.png)

Below are the performance metrics:

![Load Testing2](https://github.com/Mushroom-Wang/news-sentiment-analysis/blob/77f22b63c53ce749c4c617c156740e70d139b83d/Load%20Testing2.png)

## Reference
[DuckDuckGo API](https://serpapi.com/duckduckgo-news-results)
[Hugging Face RoBERTa](https://huggingface.co/docs/transformers/model_doc/roberta)
[Rust Tokio](https://docs.rs/tokio/latest/tokio/)