use azure_search_rs::{
    client::AzureSearchClient,
    models::VectorKind,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Hotel {
    hotel_id: String,
    description: String,
    category: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service_name = std::env::var("AZURE_SEARCH_SERVICE_NAME")?;
    let index_name = std::env::var("AZURE_SEARCH_INDEX_NAME")?;
    let api_key = std::env::var("AZURE_SEARCH_API_KEY")?;

    let client = AzureSearchClient::new(service_name, api_key, "2024-07-01").unwrap();

    // Create a text query that will be vectorized server-side
    let text_query = "luxury hotel with ocean view";
    let semantic_config = "hotel-semantic-config";

    // Create vector queries
    let vector_queries = vec![
        VectorKind::TextQuery {
            kind: "text".to_string(),
            text: text_query.to_string(),
            fields: vec!["description_vector".to_string()],
            k: 10,
        }.into()
    ];

    // Execute hybrid search
    let results = client
        .hybrid_search::<Hotel>(
            &index_name,
            text_query,
            semantic_config,
            vector_queries,
        )
        .await?;

    println!("Found {} results:", results.value.len());
    for search_result in results.value {
        let hotel: Hotel = search_result.document;
        println!("- {}: {}", hotel.hotel_id, hotel.category);
        println!("  {}", hotel.description);
        println!();
    }

    Ok(())
}
