use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Chat Completion Request
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionRequest {
    pub messages: Vec<Message>,
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Message {
    Developer {
        content: Content,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    System {
        content: Content,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    User {
        content: Content,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Content>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    Tool {
        content: Content,
        tool_call: String,
    },
    Function {
        content: Content,
        name: String,
    },
}

impl Message {
    pub fn content(&self) -> Option<&Content> {
        match self {
            Message::Assistant { content, .. } => content.as_ref(),
            Message::User { content, .. } => Some(content),
            Message::System { content, .. } => Some(content),
            Message::Developer { content, .. } => Some(content),
            Message::Tool { content, .. } => Some(content),
            Message::Function { content, .. } => Some(content),
        }
    }
    pub fn content_text(&self) -> String {
        let content = self.content().unwrap();
        match content {
            Content::Text(text) => text.clone(),
            Content::Array(_) => "<Array>".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    Array(Vec<Value>),
}
// Chat Completion Response
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: i64,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    pub system_fingerprint: String,
    pub object: String,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
    pub logprobs: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: i32,
    pub prompt_tokens: i32,
    pub total_tokens: i32,
    pub completion_tokens_details: Value,
    pub prompt_tokens_details: Value,
}

#[derive(Clone)]
pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn chat(
        &self,
        request: OpenAIChatCompletionRequest,
    ) -> Result<OpenAIChatCompletionResponse> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }

        let response_body = response.json::<OpenAIChatCompletionResponse>().await?;
        Ok(response_body)
    }
}

impl Default for OpenAIChatCompletionRequest {
    fn default() -> Self {
        Self {
            model: "gpt-4o-mini".to_string(), // Default model
            messages: Vec::new(),             // Empty messages vector
            temperature: None,
            max_tokens: None,
            max_completion_tokens: None,
            stream: None,
            user: None,
            extra: None,
        }
    }
}

impl OpenAIChatCompletionRequest {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            ..Default::default()
        }
    }

    pub fn with_message(mut self, role: impl Into<String>, content: impl Into<String>) -> Self {
        let role = role.into();
        let content = content.into();
        self.messages.push(Message::new(role, content));
        self
    }
}

impl Message {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        let role = role.into();
        match role.as_str() {
            "user" => Message::User {
                content: Content::Text(content.into()),
                name: None,
            },
            "system" => Message::System {
                content: Content::Text(content.into()),
                name: None,
            },
            "assistant" => Message::Assistant {
                content: Some(Content::Text(content.into())),
                name: None,
                extra: HashMap::new(),
            },
            "developer" => Message::Developer {
                content: Content::Text(content.into()),
                name: None,
            },
            _ => panic!("Invalid role: {}", role),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_parse_chat_completion_request() {
        let request_json = json!({
            "model": "gpt-4o",
            "messages": [
                {
                    "role": "developer",
                    "content": "You are a helpful assistant."
                },
                {
                    "role": "user",
                    "content": "Hello!"
                }
            ]
        });

        let request: OpenAIChatCompletionRequest = serde_json::from_value(request_json.clone())
            .expect("Failed to parse ChatCompletionRequest");

        assert_eq!(request.model, "gpt-4o");
        assert_eq!(request.messages.len(), 2);

        match &request.messages[0] {
            Message::Developer { content, .. } => {
                assert_eq!(
                    content,
                    &Content::Text("You are a helpful assistant.".to_string())
                );
            }
            _ => panic!("Expected Developer message"),
        }

        match &request.messages[1] {
            Message::User { content, .. } => {
                assert_eq!(content, &Content::Text("Hello!".to_string()));
            }
            _ => panic!("Expected User message"),
        }

        // Serialize back to JSON and compare
        let serialized =
            serde_json::to_value(&request).expect("Failed to serialize ChatCompletionResponse");
        assert_eq!(request_json, serialized);
    }

    #[test]
    fn test_parse_chat_completion_response() {
        let response_json = json!({
            "id": "chatcmpl-123456",
            "object": "chat.completion",
            "created": 1728933352,
            "model": "gpt-4o-2024-08-06",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": "Hi there! How can I assist you today?",
                        "refusal": null
                    },
                    "logprobs": null,
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 19,
                "completion_tokens": 10,
                "total_tokens": 29,
                "prompt_tokens_details": {
                    "cached_tokens": 0
                },
                "completion_tokens_details": {
                    "reasoning_tokens": 0,
                    "accepted_prediction_tokens": 0,
                    "rejected_prediction_tokens": 0
                }
            },
            "system_fingerprint": "fp_6b68a8204b"
        });

        let response: OpenAIChatCompletionResponse = serde_json::from_value(response_json.clone())
            .expect("Failed to parse ChatCompletionResponse");

        assert_eq!(response.id, "chatcmpl-123456");
        assert_eq!(response.model, "gpt-4o-2024-08-06");
        assert_eq!(response.created, 1728933352);
        assert_eq!(response.system_fingerprint, "fp_6b68a8204b");
        assert_eq!(response.object, "chat.completion");

        let choice = &response.choices[0];
        assert_eq!(choice.index, 0);
        assert_eq!(choice.finish_reason, "stop");
        assert_eq!(choice.logprobs, None);

        if let Message::Assistant { content, extra, .. } = &choice.message {
            assert_eq!(
                content.as_ref().unwrap(),
                &Content::Text("Hi there! How can I assist you today?".to_string())
            );
            println!("{:?}", extra["refusal"].as_str());
            assert!(extra["refusal"].is_null())
        } else {
            panic!("Expected Assistant message");
        }

        assert_eq!(response.usage.prompt_tokens, 19);
        assert_eq!(response.usage.completion_tokens, 10);
        assert_eq!(response.usage.total_tokens, 29);

        // Serialize back to JSON and compare
        let serialized =
            serde_json::to_value(&response).expect("Failed to serialize ChatCompletionResponse");
        assert_eq!(response_json, serialized);
    }
}
