use serde::{Deserialize, Serialize};

/// Represents a document in Azure AI Search
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    /// The document ID (key field)
    #[serde(rename = "id")]
    pub id: String,
    /// The document fields
    #[serde(flatten)]
    pub fields: serde_json::Value,
}
