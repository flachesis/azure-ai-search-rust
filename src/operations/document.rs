use crate::{client::AzureSearchClient, error::Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

impl AzureSearchClient {
    /// Get a document by ID
    pub async fn get_document<T>(&self, index_name: &str, document_id: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let path = format!("indexes/{}/docs/{}", index_name, document_id);
        self.send_request(reqwest::Method::GET, &path, None::<&()>)
            .await
    }

    /// Upload or update documents in an index
    pub async fn put_documents<T>(&self, index_name: &str, documents: Vec<T>) -> Result<()>
    where
        T: Serialize,
    {
        let path = format!("indexes/{}/docs/index", index_name);
        let body = json!({
            "value": documents
        });
        self.send_request::<serde_json::Value, _>(reqwest::Method::POST, &path, Some(&body))
            .await?;
        Ok(())
    }
}
