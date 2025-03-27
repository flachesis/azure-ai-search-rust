use reqwest::{Client, Url};
use serde::de::DeserializeOwned;

use crate::error::Result;

/// Main client for interacting with Azure AI Search
#[derive(Debug, Clone)]
pub struct AzureSearchClient {
    /// Base URL for the search service (e.g., https://myservice.search.windows.net)
    endpoint: Url,
    /// API version to use (e.g., "2023-11-01")
    api_version: String,
    /// Admin API key for authentication
    api_key: String,
    /// Reusable HTTP client
    http_client: Client,
}

impl AzureSearchClient {
    /// Create a new AzureSearchClient
    pub fn new(
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
        api_version: impl Into<String>,
    ) -> Result<Self> {
        let endpoint = Url::parse(&endpoint.into())?;
        Ok(Self {
            endpoint,
            api_version: api_version.into(),
            api_key: api_key.into(),
            http_client: Client::new(),
        })
    }

    /// Helper method for making authenticated requests
    pub async fn send_request<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let url = self.endpoint.join(path)?;
        let mut request = self
            .http_client
            .request(method, url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .query(&[("api-version", &self.api_version)]);

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(crate::error::Error::RequestFailed { status, body });
        }

        serde_json::from_str(&body).map_err(Into::into)
    }
}
