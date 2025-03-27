use azure_search_rs::client::AzureSearchClient;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Hotel {
    id: String,
    description_vector: Vec<f32>,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 創建client實例
    let service_name = std::env::var("AZURE_SEARCH_SERVICE_NAME")?;
    let endpoint = format!("https://{}.search.windows.net", service_name);
    let index_name = std::env::var("AZURE_SEARCH_INDEX_NAME")?;
    let api_key = std::env::var("AZURE_SEARCH_API_KEY")?;

    let client = AzureSearchClient::new(&endpoint, &api_key, "2024-07-01")?;

    // 準備測試文檔
    let documents = vec![
        Hotel {
            id: "1".to_string(),
            description_vector: vec![0.1, 0.2, 0.3, 0.4],
            description: "luxury hotel".to_string(),
        },
        Hotel {
            id: "2".to_string(),
            description_vector: vec![0.5, 0.6, 0.7, 0.8],
            description: "經濟型酒店".to_string(),
        },
    ];

    // 上傳文檔
    client.put_documents(&index_name, documents).await?;

    println!("文檔上傳成功");
    Ok(())
}
