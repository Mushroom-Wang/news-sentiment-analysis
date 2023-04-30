from duckduckgo_search import ddg_news


class DDGNewsSearch:
    def __init__(self, config=None):
        if config is None:
            config = dict(safesearch="Off", time="y", max_results=5)
        self.config = config

    def set_config(self, config):
        self.config = config

    def get_results(self, keyword: str):
        return ddg_news(keyword, **self.config)
