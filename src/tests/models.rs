use crate::models::{SearchResponse, SearchResult, SearchResultHierarchy};
use anyhow::Result;
use serde_json::json;

#[test]
fn search_result_requires_object_type_url_and_root_hierarchy() {
    let missing_object_id = json!({
        "type": "content",
        "url": "https://laravel.com/docs",
        "hierarchy": {"lvl0": "Docs"}
    });
    let missing_lvl0 = json!({
        "objectID": "docs",
        "type": "content",
        "url": "https://laravel.com/docs",
        "hierarchy": {}
    });

    assert!(serde_json::from_value::<SearchResult>(missing_object_id).is_err());
    assert!(serde_json::from_value::<SearchResult>(missing_lvl0).is_err());
}

#[test]
fn hierarchy_last_and_values_follow_deepest_populated_level() -> Result<()> {
    let hierarchy: SearchResultHierarchy = serde_json::from_value(json!({
        "lvl0": "Docs",
        "lvl1": "Requests",
        "lvl2": null,
        "lvl3": "Sparse level",
        "lvl4": null,
        "lvl5": "Deepest",
        "lvl6": null
    }))?;

    assert_eq!(hierarchy.last(), "Deepest");
    assert_eq!(
        hierarchy.values().collect::<Vec<_>>(),
        vec!["Docs", "Requests", "Sparse level", "Deepest"]
    );
    Ok(())
}

#[test]
fn search_response_keeps_provider_order_and_optional_content() -> Result<()> {
    let response: SearchResponse = serde_json::from_value(json!({
        "hits": [
            {
                "objectID": "first",
                "type": "content",
                "url": "https://laravel.com/docs/first",
                "hierarchy": {"lvl0": "First"},
                "content": "text"
            },
            {
                "objectID": "second",
                "type": "lvl1",
                "url": "https://laravel.com/docs/second",
                "hierarchy": {"lvl0": "Second", "lvl1": "Nested"}
            }
        ]
    }))?;

    assert_eq!(
        response
            .hits
            .iter()
            .map(|hit| hit.object_id.as_str())
            .collect::<Vec<_>>(),
        vec!["first", "second"]
    );
    assert_eq!(response.hits[0].content.as_deref(), Some("text"));
    assert_eq!(response.hits[1].content, None);
    Ok(())
}
