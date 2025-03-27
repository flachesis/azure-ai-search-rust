//! Azure AI Search client library for Rust
//!
//! Provides async Rust bindings for Azure AI Search (formerly Azure Cognitive Search)

pub mod client;
pub mod error;
pub mod models;
pub mod operations;

pub use client::AzureSearchClient;
pub use error::{Error, Result};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::AzureSearchClient;
}
