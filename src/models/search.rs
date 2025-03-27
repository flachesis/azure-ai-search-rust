use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QueryType {
    /// simple
    Simple,
    /// full
    Full,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SearchMode {
    /// any
    Any,
    /// all
    All,
}

/// Search request parameters
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub count: bool,
    /// Select fields
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_vec_as_string")]
    pub select: Option<Vec<String>>,
    /// Search text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Search fields
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_option_vec_as_string")]
    pub search_fields: Option<Vec<String>>,
    /// Search mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_mode: Option<SearchMode>,
    /// Filter expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Facets to include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<String>>,
    /// Vector query for vector search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_queries: Option<Vec<VectorKind>>,
    /// Semantic configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_configuration: Option<String>,
    /// Query type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<QueryType>,
    /// Orderby_expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderby: Option<String>,
    /// % of index that must be covered to declare query successful
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_coverage: Option<u8>,
    /// highlight fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<Vec<String>>,
    /// Highlight pre/post tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_post_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_pre_tag: Option<String>,
    /// Session id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Top k results to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<u16>,
    /// Skip first n results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VectorKind {
    /// Pure vector query
    VectorQuery {
        kind: String,
        vector: Vec<f32>,
        k: u8,
        #[serde(serialize_with = "serialize_vec_as_string")]
        fields: Vec<String>,
    },
    /// Text query that will be vectorized server-side
    TextQuery {
        kind: String,
        text: String,
        #[serde(serialize_with = "serialize_vec_as_string")]
        fields: Vec<String>,
        k: u8,
    },
}

/// Search result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult<T> {
    /// Document score
    #[serde(rename = "@search.score")]
    pub score: f64,
    /// Document highlights (if requested)
    #[serde(rename = "@search.highlights", skip_serializing_if = "Option::is_none")]
    pub highlights: Option<serde_json::Value>,
    /// The document itself
    #[serde(flatten)]
    pub document: T,
}

/// Search response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResponse<T> {
    /// Search results
    pub value: Vec<SearchResult<T>>,
    /// Facet results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<serde_json::Value>,
    /// Semantic answers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<serde_json::Value>,
    /// Count of total results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

/// Serialize a vector as a string
fn serialize_vec_as_string<S>(vec: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let joined = vec.join(",");
    serializer.serialize_str(&joined)
}

/// Serialize a option vector as a string
fn serialize_option_vec_as_string<S>(vec: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let joined = vec.as_ref().unwrap_or(&vec![]).join(",");
    serializer.serialize_str(&joined)
}
