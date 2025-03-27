use serde::{Deserialize, Serialize};

/// Search request parameters
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    /// Search text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Facets to include
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<String>,
    /// Vector query for vector search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_queries: Option<Vec<VectorKind>>,
    /// Semantic configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_configuration: Option<String>,
}

/// Serialize a vector as a string
fn serialize_vec_as_string<S>(vec: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let joined = vec.join(",");
    serializer.serialize_str(&joined)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VectorKind {
    /// Pure vector query
    VectorQuery {
        kind: String,
        vector: Vec<f32>,
        k: u32,
        #[serde(serialize_with = "serialize_vec_as_string")]
        fields: Vec<String>,
    },
    /// Text query that will be vectorized server-side
    TextQuery {
        kind: String,
        text: String,
        #[serde(serialize_with = "serialize_vec_as_string")]
        fields: Vec<String>,
        k: u32,
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
