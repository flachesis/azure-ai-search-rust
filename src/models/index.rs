use serde::{Deserialize, Serialize};
use crate::models::Suggester;

/// Azure Search index definition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Index {
    /// Index name
    pub name: String,
    /// Fields in the index
    pub fields: Vec<Field>,
    /// Scoring profiles
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scoring_profiles: Vec<ScoringProfile>,
    /// Suggesters
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggesters: Vec<Suggester>,
    /// CORS options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_options: Option<CorsOptions>,
}

/// Field definition for an index
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    /// Field name
    pub name: String,
    /// Field type
    #[serde(rename = "type")]
    pub field_type: FieldType,
    /// Whether the field is searchable
    pub searchable: bool,
    /// Whether the field is filterable
    pub filterable: bool,
    /// Whether the field is sortable
    pub sortable: bool,
    /// Whether the field is facetable
    pub facetable: bool,
    /// Whether the field is a key
    pub key: bool,
    /// Whether the field is retrievable
    pub retrievable: bool,
    /// Analyzer for the field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    /// Search analyzer for the field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
}

/// Field types supported by Azure Search
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Int32,
    Int64,
    Double,
    Boolean,
    DateTimeOffset,
    GeographyPoint,
    Complex,
}

/// Scoring profile for custom ranking
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoringProfile {
    pub name: String,
    pub text: Option<TextWeights>,
    pub functions: Vec<ScoringFunction>,
}

/// Text weights for scoring
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextWeights {
    pub weights: std::collections::HashMap<String, f64>,
}

/// Scoring function
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ScoringFunction {
    #[serde(rename = "magnitude")]
    Magnitude {
        field_name: String,
        boost: f64,
        parameters: MagnitudeParameters,
    },
    #[serde(rename = "freshness")]
    Freshness {
        field_name: String,
        boost: f64,
        parameters: FreshnessParameters,
    },
    #[serde(rename = "distance")]
    Distance {
        field_name: String,
        boost: f64,
        parameters: DistanceParameters,
    },
    #[serde(rename = "tag")]
    Tag {
        field_name: String,
        boost: f64,
        parameters: TagParameters,
    },
}

/// Magnitude function parameters
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MagnitudeParameters {
    pub boosting_range_start: f64,
    pub boosting_range_end: f64,
    pub constant_boost_beyond_range: bool,
}

/// Freshness function parameters
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FreshnessParameters {
    pub boosting_duration: String,
}

/// Distance function parameters
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistanceParameters {
    pub reference_point_parameter: String,
    pub boosting_distance: f64,
}

/// Tag function parameters
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagParameters {
    pub tags_parameter: String,
}

/// CORS options for the index
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CorsOptions {
    pub allowed_origins: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_in_seconds: Option<i64>,
}

/// Index statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexStatistics {
    pub document_count: i64,
    pub storage_size: i64,
}
