use reqwest::{Client};
use serde::{Serialize,Deserialize};
use async_trait::async_trait;
use crate::services::provider_trait::{ChatRequest, ChatResponse, LLMProvider, ProviderMessage, Usage};

pub struct OpenaiProvider {
    client: Client,
    url: String
}

#[derive(Serialize)]
pub struct OpenaiRequest {
    model:String,
    messages:Vec<ProviderMessage>,
    max_tokens:u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct OpenaiMessage {
    role:String,
    content:String
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct ChoicesBlock {
    message:OpenaiMessage,
    finish_reason:String
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct OpenaiUsage {
    prompt_tokens:u32,
    completion_tokens:u32
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct OpenaiResponse {
    id:String,
    usage:OpenaiUsage,
    model:String,
    choices: Vec<ChoicesBlock>,
}


impl OpenaiProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            url:"https://api.openai.com/v1/chat/completions".to_string()
        }
    }
}

#[async_trait]
impl LLMProvider for OpenaiProvider {
    async fn chat_completion(&self,api_key:&str,request:ChatRequest) -> Result<ChatResponse,String> {
        print!("fdsjfsijfsd1");
        let created_request = OpenaiRequest {
            model:request.model,
            max_tokens:request.max_tokens.unwrap_or(4096),
            temperature:request.temperature,
            messages: request.messages
        };

        let json_body = serde_json::to_string(&created_request).unwrap();

        let response = self.client
            .post(&self.url)
            .header("Authorization", format!("Bearer {}",api_key))
            .header("Content-Type", "application/json")
            .body(json_body).send().await.map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("OpenAI API error: {}", error_text));
        }

        let openai_response:OpenaiResponse = response.json().await.map_err(|e| e.to_string())?; 

        Ok(ChatResponse {
            id:openai_response.id,
            model:openai_response.model,
            content:openai_response.choices[0].message.content.clone(),
            stop_reason:openai_response.choices[0].finish_reason.clone(),
            usage: Usage {
                prompt_tokens:openai_response.usage.prompt_tokens,
                completion_tokens:openai_response.usage.completion_tokens,
                total_tokens:openai_response.usage.prompt_tokens + openai_response.usage.completion_tokens
            }
        })
    }

    fn provider_name(&self) ->  &str {
        "openai"
    }
}