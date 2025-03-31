use azure_search_rs::{client::AzureSearchClient, models::search::VectorKind, operations::search::SearchTrait};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration from environment variables
    let service_name = env::var("AZURE_SEARCH_SERVICE_NAME")
        .expect("AZURE_SEARCH_SERVICE_NAME environment variable not set");
    let api_key = env::var("AZURE_SEARCH_API_KEY")
        .expect("AZURE_SEARCH_API_KEY environment variable not set");
    let index_name = env::var("AZURE_SEARCH_INDEX_NAME")
        .expect("AZURE_SEARCH_INDEX_NAME environment variable not set");

    // Initialize client
    let endpoint = format!("https://{}.search.windows.net", service_name);
    let api_version = "2024-07-01".to_string();
    let client = AzureSearchClient::new(endpoint, api_key, api_version, None)?;

    // Example 1: Pure vector search
    let vector_query = VectorKind::VectorQuery {
        kind: "vector".to_string(),
        vector: vec![0.1, 0.2, 0.3, 0.4], // Example embedding vector
        fields: vec!["description_vector".to_string()],
        k: 5,
        weight: Some(100.0),
    };

    println!("Running pure vector search...");
    let vector_results = client
        .vector_search::<serde_json::Value>(&index_name, vec![vector_query])
        .await?;
    print_results(&vector_results);

    // Example 2: Semantic search with vector
    let text_query = VectorKind::TextQuery {
        kind: "text".to_string(),
        text: "luxury hotel with ocean view".to_string(),
        fields: vec!["description_vector".to_string()],
        k: 3,
        weight: Some(100.0),
    };

    println!("\nRunning semantic search with vector...");
    let semantic_results = client
        .semantic_search::<serde_json::Value>(
            &index_name,
            "luxury hotel",
            "my-semantic-config",
            Some(vec![text_query.clone().into()]),
        )
        .await?;
    print_results(&semantic_results);

    // Example 3: Hybrid search
    println!("\nRunning hybrid search...");
    let hybrid_results = client
        .hybrid_search::<serde_json::Value>(
            &index_name,
            "luxury hotel",
            "my-semantic-config",
            vec![text_query.into()],
        )
        .await?;
    print_results(&hybrid_results);

    Ok(())
}

fn print_results(results: &azure_search_rs::models::search::SearchResponse<serde_json::Value>) {
    println!("Found {} results:", results.value.len());
    for (i, result) in results.value.iter().enumerate() {
        println!("{}. Score: {}", i + 1, result.score);
        match &result.document {
            serde_json::Value::Object(doc) => {
                for (key, value) in doc {
                    println!("   {}: {}", key, value);
                }
            }
            _ => println!("   [Non-object document]"),
        }
    }
}
