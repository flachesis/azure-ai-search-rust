use azure_search_rs::client::AzureSearchClient;
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
    // 1. 建立AzureSearchClient
    let client = AzureSearchClient::new(endpoint, api_key, "2024-07-01".to_string(), None)?;

    // 2. 指定index名稱和document ID
    let index_name = env::var("AZURE_SEARCH_INDEX_NAME")
        .expect("AZURE_SEARCH_INDEX_NAME environment variable not set");
    let document_id = "1";

    // 3. 獲取文件
    let document: Hotel = client.get_document(&index_name, document_id).await?;

    // 4. 輸出結果
    println!("獲取到的文件內容:");
    println!("{}", serde_json::to_string_pretty(&document)?);

    Ok(())
}
