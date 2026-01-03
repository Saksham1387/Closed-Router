use reqwest::Client;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::services::provider_trait::{ChatResponse, LLMProvider,ChatRequest};
use super::provider_trait::*;

pub struct AnthropicProvider {
    client:Client,
    url:String
}

const VALID_MODELS_ANTHROPIC: &[&str] = &[
    "claude-sonnet-4-5-20250929",
    "claude-haiku-4-5-20251001",
    "claude-opus-4-5-20251101",
    "claude-opus-4-1-20250805",
    "claude-sonnet-4-20250514",
    "claude-3-7-sonnet-20250219",
    "claude-opus-4-20250514",
    "claude-3-haiku-20240307"
];

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<ProviderMessage>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct ContentBlock {
    text:String,
    #[serde(rename = "type")]
    content_type:String
}

#[derive(Deserialize)]
#[derive(Debug)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32
}

#[derive(Deserialize)]
#[derive(Debug)]
struct AnthropicResponse {
    id: String,
    model: String,
    content: Vec<ContentBlock>,
    stop_reason: String,
    usage: AnthropicUsage,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        Self {
            client : Client::new(),
            url: "https://api.anthropic.com/v1/messages".to_string()
        }
    }
 }


#[async_trait]
impl LLMProvider for AnthropicProvider {
    async fn chat_completion(&self,api_key: &str,request: ChatRequest) -> Result<ChatResponse,String>{

        if !VALID_MODELS_ANTHROPIC.contains(&request.model.as_str()) {
            return Err(format!("Unsupported model"));
        }

        let created_request = AnthropicRequest {
            model:request.model,
            messages:request.messages,
            max_tokens:request.max_tokens.unwrap_or(4096),
            temperature:request.temperature
        };

        let json_body = serde_json::to_string(&created_request).unwrap();


        let response = self.client
            .post(&self.url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .body(json_body).send().await.map_err(|e| format!("Request failed: {}", e))?;


        if !response.status().is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Anthropic API error: {}", error_text));
        }


        let anthropic_response: AnthropicResponse = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;


        println!("{:?}",anthropic_response);

        let content = anthropic_response.content
            .iter()
            .find(|c| c.content_type == "text")
            .map(|c| c.text.clone())
            .unwrap_or_default();
        

        Ok(ChatResponse{
            id: anthropic_response.id,
            model: anthropic_response.model,
            content,
            stop_reason:anthropic_response.stop_reason,
            usage: Usage {
                prompt_tokens: anthropic_response.usage.input_tokens,
                completion_tokens: anthropic_response.usage.output_tokens,
                total_tokens: anthropic_response.usage.input_tokens 
                    + anthropic_response.usage.output_tokens,
            },
        })
    }

    fn provider_name(&self) ->  &str {
        "anthropic"
    }
}