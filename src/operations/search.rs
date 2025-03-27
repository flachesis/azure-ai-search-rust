use crate::{
    client::AzureSearchClient,
    error::Result,
    models::search::{SearchRequest, SearchResponse, VectorKind},
};

impl AzureSearchClient {
    /// Execute a search query against an index
    pub async fn search<T: serde::de::DeserializeOwned>(
        &self,
        index_name: &str,
        request: &SearchRequest,
    ) -> Result<SearchResponse<T>> {
        let path = format!("indexes/{}/docs/search", index_name);
        println!("Requesting path: {}", path);
        println!("Request body: {}", serde_json::to_string(request).unwrap());
        self.send_request::<SearchResponse<T>, SearchRequest>(
            reqwest::Method::POST,
            &path,
            Some(request),
        )
        .await
    }

    /// Execute a vector search query
    pub async fn vector_search<T: serde::de::DeserializeOwned>(
        &self,
        index_name: &str,
        vector_queries: Vec<VectorKind>,
    ) -> Result<SearchResponse<T>> {
        let request = SearchRequest {
            vector_queries: Some(vector_queries),
            ..Default::default()
        };
        self.search(index_name, &request).await
    }

    /// Execute a semantic search query (optionally with vector search)
    pub async fn semantic_search<T: serde::de::DeserializeOwned>(
        &self,
        index_name: &str,
        query: &str,
        semantic_configuration: &str,
        vector_queries: Option<Vec<VectorKind>>,
    ) -> Result<SearchResponse<T>> {
        let request = SearchRequest {
            search: Some(query.to_string()),
            semantic_configuration: Some(semantic_configuration.to_string()),
            vector_queries,
            ..Default::default()
        };
        self.search(index_name, &request).await
    }

    /// Execute a hybrid search combining semantic and vector search
    pub async fn hybrid_search<T: serde::de::DeserializeOwned>(
        &self,
        index_name: &str,
        query: &str,
        semantic_configuration: &str,
        vector_queries: Vec<VectorKind>,
    ) -> Result<SearchResponse<T>> {
        self.semantic_search(
            index_name,
            query,
            semantic_configuration,
            Some(vector_queries),
        )
        .await
    }
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            search: None,
            filter: None,
            facets: Vec::new(),
            vector_queries: None,
            semantic_configuration: None,
        }
    }
}
