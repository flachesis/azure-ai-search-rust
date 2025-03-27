use crate::{
    client::AzureSearchClient,
    error::Result,
    models::suggest::{SuggestParameters, SuggestResult},
};

impl AzureSearchClient {
    /// Get search suggestions
    pub async fn suggest(
        &self,
        index_name: &str,
        params: &SuggestParameters,
    ) -> Result<SuggestResult> {
        let path = format!("indexes/{}/docs/suggest", index_name);
        self.send_request::<SuggestResult, SuggestParameters>(
            reqwest::Method::POST,
            &path,
            Some(params),
        )
        .await
    }
}
