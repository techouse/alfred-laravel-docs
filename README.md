# Laravel Docs Workflow for Alfred

![GitHub release](https://img.shields.io/github/release/techouse/alfred-laravel-docs.svg)
![GitHub All Releases](https://img.shields.io/github/downloads/techouse/alfred-laravel-docs/total.svg)
![GitHub](https://img.shields.io/github/license/techouse/alfred-laravel-docs.svg)

Search the [Laravel documentation](https://laravel.com/docs) using [Alfred](https://www.alfredapp.com/).

![demo](demo.gif)

## Installation

1. [Download the latest version](https://github.com/techouse/alfred-laravel-docs/releases/latest)
2. Install the workflow by double-clicking the `.alfredworkflow` file
3. You can add the workflow to a category, then click "Import" to finish importing. You'll now see the workflow listed in the left sidebar of your Workflows preferences pane.

## Usage

Just type `ld` followed by your search query.

```
ld request
```

Either press `⌘Y` to Quick Look the result, or press `<enter>` to open it in your web browser.

## Changing Branches

The workflow supports searching the documentation of all the currently supported branches: `v10`, `v9`, `v8`, `v7`, `v6`, `v5.8`, `v5.7`, `v5.6`, `v5.5`, `v5.4`, `v5.3`, `v5.2`, `v5.1`, `v5` and `v4.2`.

By default, it searches the `v10` branch. 

To search branch `v9` simply type `v9` **anywhere** in your query, like so:
```
ld request v9
```

### Note

Kudos to [tillkruss/alfred-laravel-docs](https://github.com/tillkruss/alfred-laravel-docs) for the initial inspiration.

The lightning fast search is powered by [Algolia](https://www.algolia.com) using the same index as the official [Laravel Docs](https://laravel.com/docs/) website.