use serde::Deserialize;

/// An Algolia result returned by the Laravel documentation index.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SearchResult {
    /// Algolia object identifier.
    #[serde(rename = "objectID")]
    pub object_id: String,
    /// Algolia record type.
    #[serde(rename = "type")]
    pub result_type: String,
    /// Documentation URL opened by Alfred.
    pub url: String,
    /// Documentation hierarchy used for titles and breadcrumbs.
    pub hierarchy: SearchResultHierarchy,
    /// Optional searchable page content.
    pub content: Option<String>,
}

/// Ordered hierarchy values returned for a documentation result.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SearchResultHierarchy {
    /// Root hierarchy value.
    pub lvl0: String,
    /// Level-one hierarchy value.
    pub lvl1: Option<String>,
    /// Level-two hierarchy value.
    pub lvl2: Option<String>,
    /// Level-three hierarchy value.
    pub lvl3: Option<String>,
    /// Level-four hierarchy value.
    pub lvl4: Option<String>,
    /// Level-five hierarchy value.
    pub lvl5: Option<String>,
    /// Level-six hierarchy value.
    pub lvl6: Option<String>,
}

impl SearchResultHierarchy {
    /// Returns the deepest populated hierarchy value.
    pub fn last(&self) -> &str {
        self.lvl6
            .as_deref()
            .or(self.lvl5.as_deref())
            .or(self.lvl4.as_deref())
            .or(self.lvl3.as_deref())
            .or(self.lvl2.as_deref())
            .or(self.lvl1.as_deref())
            .unwrap_or(&self.lvl0)
    }

    /// Iterates over populated hierarchy values in level order.
    pub fn values(&self) -> impl Iterator<Item = &str> {
        [
            Some(self.lvl0.as_str()),
            self.lvl1.as_deref(),
            self.lvl2.as_deref(),
            self.lvl3.as_deref(),
            self.lvl4.as_deref(),
            self.lvl5.as_deref(),
            self.lvl6.as_deref(),
        ]
        .into_iter()
        .flatten()
    }
}

/// Minimal subset of an Algolia single-index search response.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SearchResponse {
    /// Search results in provider-defined ranking order.
    pub hits: Vec<SearchResult>,
}
