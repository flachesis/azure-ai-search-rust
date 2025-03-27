use crate::{
    client::AzureSearchClient,
    error::Result,
    models::index::{Index, IndexStatistics},
};

impl AzureSearchClient {
    /// Create a new index
    pub async fn create_index(&self, index: &Index) -> Result<()> {
        self.send_request::<(), Index>(reqwest::Method::POST, "indexes", Some(index))
            .await?;
        Ok(())
    }

    /// Delete an index
    pub async fn delete_index(&self, index_name: &str) -> Result<()> {
        let path = format!("indexes/{}", index_name);
        self.send_request::<(), ()>(reqwest::Method::DELETE, &path, None::<&()>)
            .await?;
        Ok(())
    }

    /// Get an index definition
    pub async fn get_index(&self, index_name: &str) -> Result<Index> {
        let path = format!("indexes/{}", index_name);
        self.send_request::<Index, ()>(reqwest::Method::GET, &path, None::<&()>)
            .await
    }

    /// Get index statistics
    pub async fn get_index_stats(&self, index_name: &str) -> Result<IndexStatistics> {
        let path = format!("indexes/{}/stats", index_name);
        self.send_request::<IndexStatistics, ()>(reqwest::Method::GET, &path, None::<&()>)
            .await
    }

    /// List all indexes
    pub async fn list_indexes(&self) -> Result<Vec<Index>> {
        let response = self
            .send_request::<serde_json::Value, ()>(reqwest::Method::GET, "indexes", None::<&()>)
            .await?;
        Ok(serde_json::from_value(response["value"].clone())?)
    }

    /// Update an existing index
    pub async fn update_index(&self, index: &Index) -> Result<()> {
        let path = format!("indexes/{}", index.name);
        self.send_request::<(), Index>(reqwest::Method::PUT, &path, Some(index))
            .await?;
        Ok(())
    }
}
