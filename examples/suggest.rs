use azure_search_rs::{
    client::AzureSearchClient,
    models::suggest::{SuggestParameters, SuggestResult},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration from environment variables
    let service_name = env::var("AZURE_SEARCH_SERVICE_NAME")
        .expect("AZURE_SEARCH_SERVICE_NAME environment variable not set");
    let api_key = env::var("AZURE_SEARCH_API_KEY")
        .expect("AZURE_SEARCH_API_KEY environment variable not set");

    // Initialize client
    let endpoint = format!("https://{}.search.windows.net", service_name);
    let api_version = "2023-11-01".to_string();
    let client = AzureSearchClient::new(endpoint, api_key, api_version)?;

    // Example 1: Basic suggestion
    let params = SuggestParameters {
        search: "micro".to_string(),
        suggester_name: "sg".to_string(),
        fuzzy: None,
        highlight_post_tag: None,
        highlight_pre_tag: None,
    };

    let result = client.suggest("hotels", &params).await?;
    print_suggestions(&result);

    // Example 2: Fuzzy suggestion with highlighting
    let params = SuggestParameters {
        search: "micor".to_string(), // Intentional typo
        suggester_name: "sg".to_string(),
        fuzzy: Some(true),
        highlight_post_tag: Some("</em>".to_string()),
        highlight_pre_tag: Some("<em>".to_string()),
    };

    let result = client.suggest("hotels", &params).await?;
    print_suggestions(&result);

    Ok(())
}

fn print_suggestions(result: &SuggestResult) {
    println!("Found {} suggestions:", result.value.len());
    for suggestion in &result.value {
        println!("- {}", suggestion.text);
        if let Some(doc) = suggestion.document.as_object() {
            println!("  Document fields:");
            for (key, value) in doc {
                println!("    {}: {}", key, value);
            }
        }
    }
}
