use alfred_workflow_rs::{Icon, Item, ItemText};
use anyhow::Result;
use html_escape::decode_html_entities;
use url::Url;

use crate::models::SearchResult;

/// Builds the placeholder shown before the user enters a search query.
pub fn placeholder_item() -> Item {
    Item::new("Search the Laravel docs...").set_icon(Icon::new("icon.png"))
}

/// Converts ranked Laravel search results into Alfred items in provider order.
pub fn items_from_results(results: &[SearchResult]) -> Result<Vec<Item>> {
    results.iter().map(item_from_result).collect()
}

/// Builds the Google fallback shown when Algolia returns no hits.
pub fn google_fallback_item(query: &str) -> Result<Item> {
    let url = Url::parse_with_params(
        "https://www.google.com/search",
        [("q", format!("Laravel {query}"))],
    )?;

    Ok(Item::builder("No matching answers found")
        .subtitle("Shall I try and search Google?")
        .arg(url.as_str())
        .text(ItemText::new(url.as_str()))
        .quick_look_url(url.as_str())
        .icon(Icon::new("google.png"))
        .valid(true)
        .build()?)
}

fn item_from_result(result: &SearchResult) -> Result<Item> {
    let title = decode_html_entities(result.hierarchy.last()).into_owned();
    let raw_breadcrumb = result
        .hierarchy
        .values()
        .filter(|value| *value != title)
        .collect::<Vec<_>>()
        .join(" > ");
    let breadcrumb = decode_html_entities(&raw_breadcrumb).into_owned();
    let subtitle = truncate_breadcrumb(&breadcrumb);

    Ok(Item::builder(title.clone())
        .uid(&result.object_id)
        .subtitle(subtitle)
        .arg(&result.url)
        .text(ItemText::new(&result.url).with_large_type(title))
        .quick_look_url(&result.url)
        .icon(Icon::new("icon.png"))
        .valid(true)
        .build()?)
}

fn truncate_breadcrumb(breadcrumb: &str) -> String {
    if breadcrumb.chars().count() <= 75 {
        return breadcrumb.to_owned();
    }

    breadcrumb.chars().take(72).chain("...".chars()).collect()
}

#[cfg(test)]
#[path = "tests/app.rs"]
mod tests;
