use super::*;
use crate::models::SearchResultHierarchy;
use alfred_workflow_rs::Item;

fn result(result_type: &str) -> SearchResult {
    SearchResult {
        object_id: "request-body".to_owned(),
        result_type: result_type.to_owned(),
        url: "https://laravel.com/docs/requests#request-body".to_owned(),
        hierarchy: SearchResultHierarchy {
            lvl0: "Docs &amp; Guides".to_owned(),
            lvl1: Some("Requests &amp; Input".to_owned()),
            lvl2: Some("Request Body".to_owned()),
            lvl3: None,
            lvl4: None,
            lvl5: None,
            lvl6: None,
        },
        content: None,
    }
}

#[test]
fn hierarchy_last_chooses_deepest_title_for_every_result_type() -> Result<()> {
    let items = items_from_results(&[result("content")])?;

    assert_eq!(items[0].title(), "Request Body");
    assert_eq!(
        items[0].subtitle(),
        Some("Docs & Guides > Requests & Input")
    );
    Ok(())
}

#[test]
fn items_from_results_preserves_provider_order() -> Result<()> {
    let mut second = result("lvl1");
    second.object_id = "second".to_owned();

    let items = items_from_results(&[result("lvl2"), second])?;

    assert_eq!(
        items.iter().map(Item::uid).collect::<Vec<_>>(),
        vec![Some("request-body"), Some("second")]
    );
    Ok(())
}

#[test]
fn item_decodes_html_title_and_breadcrumb() -> Result<()> {
    let mut item_result = result("lvl1");
    item_result.hierarchy.lvl1 = Some("Requests & Input".to_owned());
    item_result.hierarchy.lvl2 = None;

    let items = items_from_results(&[item_result])?;

    assert_eq!(items[0].title(), "Requests & Input");
    assert_eq!(items[0].subtitle(), Some("Docs & Guides"));
    Ok(())
}

#[test]
fn duplicate_title_values_are_removed_from_breadcrumb() -> Result<()> {
    let mut item_result = result("lvl2");
    item_result.hierarchy.lvl0 = "Request Body".to_owned();
    item_result.hierarchy.lvl1 = Some("Request Body".to_owned());

    let items = items_from_results(&[item_result])?;

    assert_eq!(items[0].title(), "Request Body");
    assert_eq!(items[0].subtitle(), Some(""));
    Ok(())
}

#[test]
fn empty_breadcrumb_is_rendered_as_empty_subtitle() -> Result<()> {
    let mut item_result = result("content");
    item_result.hierarchy = SearchResultHierarchy {
        lvl0: "Request Body".to_owned(),
        lvl1: None,
        lvl2: None,
        lvl3: None,
        lvl4: None,
        lvl5: None,
        lvl6: None,
    };

    let items = items_from_results(&[item_result])?;

    assert_eq!(items[0].subtitle(), Some(""));
    Ok(())
}

#[test]
fn unicode_breadcrumb_truncation_keeps_75_scalar_values() -> Result<()> {
    let mut item_result = result("lvl1");
    item_result.hierarchy.lvl0 = "é".repeat(80);
    item_result.hierarchy.lvl1 = Some("Title".to_owned());

    let item = items_from_results(&[item_result])?.remove(0);
    let subtitle = item.subtitle().expect("subtitle must be present");

    assert_eq!(subtitle.chars().count(), 75);
    assert_eq!(subtitle, format!("{}...", "é".repeat(72)));
    Ok(())
}

#[test]
fn item_preserves_url_text_and_icon_fields() -> Result<()> {
    let item = items_from_results(&[result("lvl2")])?.remove(0);

    assert_eq!(item.uid(), Some("request-body"));
    assert_eq!(
        item.arg(),
        Some("https://laravel.com/docs/requests#request-body")
    );
    assert_eq!(item.quick_look_url(), item.arg());
    assert!(item.valid());
    assert_eq!(item.icon().map(|icon| icon.path()), Some("icon.png"));
    assert_eq!(item.text().map(|text| text.copy()), item.arg());
    assert_eq!(
        item.text().and_then(|text| text.large_type()),
        Some("Request Body")
    );
    Ok(())
}

#[test]
fn google_fallback_encodes_query_and_is_selectable() -> Result<()> {
    let item = google_fallback_item("request body")?;

    assert_eq!(
        item.arg(),
        Some("https://www.google.com/search?q=Laravel+request+body")
    );
    assert_eq!(item.quick_look_url(), item.arg());
    assert_eq!(item.text().map(|text| text.copy()), item.arg());
    assert_eq!(item.icon().map(|icon| icon.path()), Some("google.png"));
    assert!(item.valid());
    Ok(())
}

#[test]
fn placeholder_is_not_selectable() {
    let item = placeholder_item();

    assert_eq!(item.title(), "Search the Laravel docs...");
    assert_eq!(item.icon().map(|icon| icon.path()), Some("icon.png"));
    assert!(!item.valid());
}
