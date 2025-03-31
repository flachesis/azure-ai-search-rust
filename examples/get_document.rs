use azure_search_rs::client::AzureSearchClient;
use azure_search_rs::operations::document::DocumentTrait;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Hotel {
    id: String,
    description_vector: Vec<f32>,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration from environment variables
    let service_name = env::var("AZURE_SEARCH_SERVICE_NAME")
        .expect("AZURE_SEARCH_SERVICE_NAME environment variable not set");
    let endpoint = format!("https://{}.search.windows.net", service_name);
    let api_key = env::var("AZURE_SEARCH_API_KEY")
        .expect("AZURE_SEARCH_API_KEY environment variable not set");
    // 1. Create AzureSearchClient
    let client = AzureSearchClient::new(endpoint, api_key, "2024-07-01".to_string(), None)?;

    // 2. Specify index name and document ID
    let index_name = env::var("AZURE_SEARCH_INDEX_NAME")
        .expect("AZURE_SEARCH_INDEX_NAME environment variable not set");
    let document_id = "1";

    // 3. Get document
    let document: Hotel = client.get_document(&index_name, document_id).await?;

    // 4. Print result
    println!("Retrieved document content:");
    println!("{}", serde_json::to_string_pretty(&document)?);

    Ok(())
}
