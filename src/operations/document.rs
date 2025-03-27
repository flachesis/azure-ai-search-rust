use crate::{client::AzureSearchClient, error::Result, models::document::Document};

impl AzureSearchClient {
    /// Get a document by ID
    pub async fn get_document(&self, index_name: &str, document_id: &str) -> Result<Document> {
        let path = format!("indexes/{}/docs/{}", index_name, document_id);
        self.send_request(reqwest::Method::GET, &path, None::<&()>)
            .await
    }
}
