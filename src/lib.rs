#![forbid(unsafe_code)]

//! Laravel documentation search and Alfred Script Filter support.

/// Conversion from Laravel search results to Alfred items.
pub mod app;
/// Search-result models returned by Algolia.
pub mod models;
/// External services used by the workflow.
pub mod services;

#[cfg(test)]
#[path = "tests/models.rs"]
mod tests;
