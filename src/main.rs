use anyhow::{Error, Result};
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use kubellm::models::openai::{self, OpenAIChatCompletionRequest, OpenAIClient};
use reqwest::StatusCode;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    client: OpenAIClient,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get API key from environment variable
    let api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in environment");
    let state = AppState {
        client: openai::OpenAIClient::new(api_key),
    };

    // Build router
    let app = Router::new()
        .route("/v1/chat/completions", post(chat_handler))
        .with_state(state);

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn chat_handler(
    State(state): State<AppState>,
    Json(request): Json<OpenAIChatCompletionRequest>,
) -> impl IntoResponse {
    let response = state.client.chat(request).await.unwrap();
    (StatusCode::OK, Json(response))
}
