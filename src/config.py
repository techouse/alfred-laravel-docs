# encoding: utf-8


class Config(object):
    # Number of results to fetch from API
    RESULT_COUNT = 9
    # How long to cache results for
    CACHE_MAX_AGE = 20  # seconds
    # Icon
    LARAVEL_ICON = "icon.png"
    GOOGLE_ICON = "google.png"
    # supported docs
    SUPPORTED_LARAVEL_VERSIONS = {
        "9.x",
        "9",
        "8.x",
        "8",
        "7.x",
        "7",
        "6.x",
        "6",
        "5.8",
        "5.7",
        "5.6",
        "5.5",
        "5.4",
        "5.3",
        "5.2",
        "5.1",
        "5.0",
        "5",
        "4.2",
    }
    SUPPORTED_LARAVEL_SPECIAL_VERSION_NAMES = {
        "9": "9.x",
        "8": "8.x",
        "7": "7.x",
        "6": "6.x",
        "5": "5.0",
    }
    DEFAULT_LARAVEL_VERSION = "9.x"
    # Algolia credentials
    ALGOLIA_APP_ID = "E3MIRNPJH5"
    ALGOLIA_SEARCH_ONLY_API_KEY = "1fa3a8fec06eb1858d6ca137211225c0"
    ALGOLIA_SEARCH_INDEX = "laravel"
