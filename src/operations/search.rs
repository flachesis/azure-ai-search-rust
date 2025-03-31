use crate::{
    client::AzureSearchClient,
    error::Result,
    models::{search::{SearchRequest, SearchResponse, VectorKind}, QueryType},
};

pub trait SearchTrait {
    /// Execute a search query against an index
    fn search<T: serde::de::DeserializeOwned>(
        &self,
        index_name: &str,
        request: &SearchRequest,
    ) -> impl std::future::Future<Output = Result<SearchResponse<T>>> + Send;

    /// Execute a vector search query
    fn vector_search<T: serde::de::DeserializeOwned>(
        &self,
        index_name: &str,
        vector_queries: Vec<VectorKind>,
    ) -> impl std::future::Future<Output = Result<SearchResponse<T>>> + Send;

    /// Execute a semantic search query (optionally with vector search)
    fn semantic_search<T: serde::de::DeserializeOwned>(
        &self,
        index_name: &str,
        query: &str,
        semantic_configuration: &str,
        vector_queries: Option<Vec<VectorKind>>,
    ) -> impl std::future::Future<Output = Result<SearchResponse<T>>> + Send;

    /// Execute a hybrid search combining semantic and vector search
    fn hybrid_search<T: serde::de::DeserializeOwned>(
        &self,
        index_name: &str,
        query: &str,
        semantic_configuration: &str,
        vector_queries: Vec<VectorKind>,
    ) -> impl std::future::Future<Output = Result<SearchResponse<T>>> + Send;
}

impl SearchTrait for AzureSearchClient {
    /// Execute a search query against an index
    async fn search<T: serde::de::DeserializeOwned>(
        &self,
        index_name: &str,
        request: &SearchRequest,
    ) -> Result<SearchResponse<T>> {
        let path = format!("indexes/{}/docs/search", index_name);
        self.send_request::<SearchResponse<T>, SearchRequest>(
            reqwest::Method::POST,
            &path,
            Some(request),
        )
        .await
    }

    /// Execute a vector search query
    async fn vector_search<T: serde::de::DeserializeOwned>(
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
    async fn semantic_search<T: serde::de::DeserializeOwned>(
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
            query_type: Some(QueryType::Semantic),
            ..Default::default()
        };
        self.search(index_name, &request).await
    }

    /// Execute a hybrid search combining semantic and vector search
    async fn hybrid_search<T: serde::de::DeserializeOwned>(
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
