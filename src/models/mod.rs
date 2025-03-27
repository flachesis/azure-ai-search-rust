//! Data models for Azure AI Search operations

pub mod document;
pub mod index;
pub mod search;
pub mod suggest;

/// Re-export commonly used models
pub use index::*;
pub use search::*;
pub use suggest::*;
