use azure_search_rs::{
    client::AzureSearchClient,
    models::{QueryType, SearchMode, SearchRequest, SearchResponse, VectorKind},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Hotel {
    id: String,
    description_vector: Option<Vec<f32>>,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_name = std::env::var("AZURE_SEARCH_SERVICE_NAME")?;
    let endpoint = format!("https://{}.search.windows.net", service_name);
    let index_name = std::env::var("AZURE_SEARCH_INDEX_NAME")?;
    let api_key = std::env::var("AZURE_SEARCH_API_KEY")?;

    let client = AzureSearchClient::new(endpoint, api_key, "2024-07-01", None).unwrap();

    // Create a text query that will be vectorized server-side
    let text_query = "luxury hotel with ocean view";
    let semantic_config = "hotel-semantic-config";

    // Create vector queries
    let vector_queries = vec![VectorKind::TextQuery {
        kind: "text".to_string(),
        text: text_query.to_string(),
        fields: vec!["description_vector".to_string()],
        k: 10,
    }
    .into()];

    // Execute hybrid search
    let results = client
        .hybrid_search::<Hotel>(
            &index_name,
            text_query,
            semantic_config,
            vector_queries.clone(),
        )
        .await?;

    println!("Found {} results:", results.value.len());
    for search_result in results.value {
        let hotel: Hotel = search_result.document;
        println!("- {}: {:?}", hotel.id, hotel.description_vector);
        println!("  {}", hotel.description);
        println!();
    }

    let results: SearchResponse<Hotel> = client
        .search(
            &index_name,
            &SearchRequest {
                count: true,
                select: Some(vec!["id".to_string(), "description".to_string()]),
                top: Some(10),
                skip: Some(0),
                search: Some(text_query.to_string()),
                query_type: Some(QueryType::Simple),
                semantic_configuration: Some(semantic_config.to_string()),
                vector_queries: Some(vector_queries),
                search_mode: Some(SearchMode::Any),
                filter: Some("description eq 'luxury hotel'".to_string()),
                orderby: Some("description desc".to_string()),
                facets: Some(vec!["description".to_string()]),
                highlight: Some(vec!["description".to_string()]),
                highlight_pre_tag: Some("<b>".to_string()),
                highlight_post_tag: Some("</b>".to_string()),
                minimum_coverage: Some(100),
                search_fields: Some(vec!["description".to_string()]),
                session_id: Some("session-id".to_string()),
            },
        )
        .await?;

    println!("Found {} results:", results.value.len());
    for search_result in results.value {
        let hotel: Hotel = search_result.document;
        println!("- {}: {:?}", hotel.id, hotel.description_vector);
        println!("  {}", hotel.description);
        println!();
    }

    Ok(())
}
