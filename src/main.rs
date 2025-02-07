use anyhow::{Error, Result};
use kubellm::models::openai;
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file
    // dotenv().ok();

    // Get API key from environment variable
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in environment");

    // Create the client
    let client = openai::OpenAIClient::new(api_key);

    // Create a simple chat request
    let request = openai::OpenAIChatCompletionRequest::new("gpt-4o-mini")
        .with_message("user", "Hello, how are you?");

    println!("Sending request to OpenAI...");

    let serialized = serde_json::to_string(&request).unwrap();
    println!("Request: {}", serialized);

    // Make the request
    match client.chat(request).await {
        Ok(response) => {
            println!("\nResponse received!");
            println!("Model: {}", response.model);
            println!(
                "Message content: {}",
                response.choices[0].message.content_text()
            );
            println!("\nUsage statistics:");
            println!("  Prompt tokens: {}", response.usage.prompt_tokens);
            println!("  Completion tokens: {}", response.usage.completion_tokens);
            println!("  Total tokens: {}", response.usage.total_tokens);
        }
        Err(e) => {
            eprintln!("Error making request: {}", e);
        }
    }

    Ok(())
}
