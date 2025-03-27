use serde::{Deserialize, Serialize};

/// Defines a suggester for fuzzy search
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Suggester {
    /// The name of the suggester
    pub name: String,
    /// The search mode for suggestions
    pub search_mode: String,
    /// The fields to use for suggestions
    pub source_fields: Vec<String>,
}

/// Parameters for suggest operation
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuggestParameters {
    /// The search text
    pub search: String,
    /// The suggester name
    pub suggester_name: String,
    /// Whether to use fuzzy matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<bool>,
    /// Highlight pre/post tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_post_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_pre_tag: Option<String>,
}

/// A single suggestion result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Suggestion {
    /// The suggestion text
    pub text: String,
    /// The document containing the suggestion
    #[serde(flatten)]
    pub document: serde_json::Value,
}

/// Response from suggest operation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuggestResult {
    /// The suggestions
    pub value: Vec<Suggestion>,
}
