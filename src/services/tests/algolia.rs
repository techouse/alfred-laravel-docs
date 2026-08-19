use super::*;
use serde_json::{Value, json};

fn config() -> AlgoliaSearchConfig {
    AlgoliaSearchConfig {
        application_id: "app".to_owned(),
        api_key: "key".to_owned(),
        index_name: "laravel".to_owned(),
    }
}

#[test]
fn endpoint_uses_single_index_search_route() -> Result<()> {
    let client = AlgoliaSearch::with_base_url(config(), Url::parse("http://127.0.0.1:8080/api/")?)?;

    assert_eq!(
        client.endpoint()?.as_str(),
        "http://127.0.0.1:8080/api/1/indexes/laravel/query"
    );
    Ok(())
}

#[test]
fn client_uses_platform_verifier_and_search_timeouts() -> Result<()> {
    let client = AlgoliaSearch::with_base_url(config(), Url::parse("http://localhost/")?)?;
    let timeouts = client.agent.config().timeouts();

    assert!(matches!(
        client.agent.config().tls_config().root_certs(),
        ureq::tls::RootCerts::PlatformVerifier
    ));
    assert_eq!(timeouts.connect, Some(CONNECT_TIMEOUT));
    assert_eq!(timeouts.global, Some(SEARCH_TIMEOUT));
    Ok(())
}

#[test]
fn request_body_uses_laravel_nine_hit_contract_without_tailwind_fields() -> Result<()> {
    let client = AlgoliaSearch::with_base_url(config(), Url::parse("http://localhost/")?)?;
    let body: Value = serde_json::from_str(&client.request_body("request body", "13.x")?)?;

    assert_eq!(
        body,
        json!({
            "query": "request body",
            "facetFilters": ["version:13.x"],
            "attributesToRetrieve": [
                "hierarchy.lvl0", "hierarchy.lvl1", "hierarchy.lvl2",
                "hierarchy.lvl3", "hierarchy.lvl4", "hierarchy.lvl5",
                "hierarchy.lvl6", "content", "type", "url"
            ],
            "page": 0,
            "hitsPerPage": 9
        })
    );
    assert!(body.get("attributesToSnippet").is_none());
    assert!(body.get("snippetEllipsisText").is_none());
    assert!(body.get("distinct").is_none());
    Ok(())
}

#[test]
fn search_response_deserializes_hierarchy_and_content() -> Result<()> {
    let response: SearchResponse = serde_json::from_value(json!({
        "hits": [{
            "objectID": "request-body",
            "type": "content",
            "url": "https://laravel.com/docs/requests",
            "hierarchy": {
                "lvl0": "Docs",
                "lvl1": "Requests",
                "lvl2": null
            },
            "content": "Request content"
        }]
    }))?;

    assert_eq!(response.hits[0].hierarchy.last(), "Requests");
    assert_eq!(response.hits[0].content.as_deref(), Some("Request content"));
    Ok(())
}

#[test]
fn empty_configuration_values_are_rejected_before_client_creation() {
    for (field, expected) in [
        ("application_id", "ALGOLIA_APPLICATION_ID must not be empty"),
        ("api_key", "ALGOLIA_SEARCH_ONLY_API_KEY must not be empty"),
        ("index_name", "ALGOLIA_SEARCH_INDEX must not be empty"),
    ] {
        let mut invalid = config();
        match field {
            "application_id" => invalid.application_id.clear(),
            "api_key" => invalid.api_key.clear(),
            "index_name" => invalid.index_name.clear(),
            _ => unreachable!("test field is fixed"),
        }

        let error = match AlgoliaSearch::new(invalid) {
            Ok(_) => panic!("empty values must fail"),
            Err(error) => error,
        };
        assert_eq!(error.to_string(), expected);
    }
}
