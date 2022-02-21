#!/usr/bin/python
# encoding: utf-8

from __future__ import print_function, unicode_literals, absolute_import

import functools
import re
import sys
from collections import OrderedDict
from HTMLParser import HTMLParser
from textwrap import wrap
from urllib import quote_plus

from algoliasearch.search_client import SearchClient
from config import Config
from workflow import Workflow3, ICON_INFO

# Algolia client
client = SearchClient.create(Config.ALGOLIA_APP_ID, Config.ALGOLIA_SEARCH_ONLY_API_KEY)
index = client.init_index(Config.ALGOLIA_SEARCH_INDEX)

# log
log = None


def cache_key(query, version=Config.DEFAULT_LARAVEL_VERSION):
    """Make filesystem-friendly cache key"""
    key = query + "_" + version
    key = key.lower()
    key = re.sub(r"[^a-z0-9-_;.]", "-", key)
    key = re.sub(r"-+", "-", key)
    # log.debug("Cache key : {!r} {!r} -> {!r}".format(query, version, key))
    return key


def handle_result(api_dict):
    """Extract relevant info from API result"""
    result = {}

    for key in {"objectID", "hierarchy", "content", "url", "anchor"}:
        if key == "hierarchy":
            api_dict[key] = OrderedDict(sorted(api_dict[key].items(), reverse=True))
            for hierarchy_key, hierarchy_value in api_dict[key].items():
                if hierarchy_value:
                    result["title"] = hierarchy_value
                    break

            result["subtitle"] = (
                " > ".join(
                    [
                        value
                        for value in OrderedDict(
                            sorted(api_dict[key].items(), key=lambda x: x[0])
                        ).values()
                        if value is not None
                    ][:-1]
                )
                if len(api_dict[key]) > 1
                else None
            )
        else:
            result[key] = api_dict[key]

    return result


def search(
    query=None, version=Config.DEFAULT_LARAVEL_VERSION, limit=Config.RESULT_COUNT
):
    if query:
        results = index.search(
            query,
            {
                "facetFilters": ["version:{}".format(version)],
                "page": 0,
                "hitsPerPage": limit,
            },
        )
        if results is not None and "hits" in results:
            return results["hits"]
    return []


def main(wf):
    if wf.update_available:
        # Add a notification to top of Script Filter results
        wf.add_item(
            "New version available",
            "Action this item to install the update",
            autocomplete="workflow:update",
            icon=ICON_INFO,
        )

    query = wf.args[0].strip()

    # Tag prefix only. Treat as blank query
    if query == "v":
        query = ""

    if not query:
        wf.add_item("Search the Laravel docs...")
        wf.send_feedback()
        return 0

    # Parse query into query string and tags
    words = query.split(" ")

    query = []
    version = Config.DEFAULT_LARAVEL_VERSION

    for word in words:
        if word.replace("v", "") in Config.SUPPORTED_LARAVEL_VERSIONS:
            version = word.replace("v", "")
            if version in Config.SUPPORTED_LARAVEL_SPECIAL_VERSION_NAMES.keys():
                version = Config.SUPPORTED_LARAVEL_SPECIAL_VERSION_NAMES[version]
        else:
            query.append(word)

    query = " ".join(query)

    # log.debug("version: {!r}".format(version))
    # log.debug("query: {!r}".format(query))

    key = cache_key(query, version)

    results = [
        handle_result(result)
        for result in wf.cached_data(
            key, functools.partial(search, query, version), max_age=Config.CACHE_MAX_AGE
        )
    ]

    # log.debug("{} results for {!r}, version {!r}".format(len(results), query, version))

    # Show results
    if not results:
        url = "https://www.google.com/search?q={}".format(
            quote_plus("Laravel {}".format(query))
        )
        wf.add_item(
            "No matching answers found",
            "Shall I try and search Google?",
            valid=True,
            arg=url,
            copytext=url,
            quicklookurl=url,
            icon=Config.GOOGLE_ICON,
        )

    html_parser = HTMLParser()

    for result in results:
        if "subtitle" in result and result["subtitle"] is not None:
            subtitle = wrap(result["subtitle"], width=75)[0]
            if len(result["subtitle"]) > 75:
                subtitle += "..."
        elif "content" in result and result["content"] is not None:
            subtitle = wrap(result["content"], width=75)[0]
            if len(result["content"]) > 75:
                subtitle += "..."
        else:
            subtitle = ""

        wf.add_item(
            uid=result["objectID"],
            title=html_parser.unescape(result["title"]),
            subtitle=html_parser.unescape(subtitle),
            arg=result["url"],
            valid=True,
            largetext=html_parser.unescape(result["title"]),
            copytext=result["url"],
            quicklookurl=result["url"],
            icon=Config.LARAVEL_ICON,
        )
        # log.debug(result)

    wf.send_feedback()


if __name__ == "__main__":
    wf = Workflow3(
        update_settings={
            "github_slug": "techouse/alfred-laravel-docs",
            "frequency": 7,
        }
    )
    log = wf.logger
    sys.exit(wf.run(main))
