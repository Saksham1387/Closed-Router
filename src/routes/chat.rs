use poem::{handler, web::{Data, Json}};
use crate::{request_inputs::ChatInput, request_outputs::{Choice, Message}, services::{anthropic::AnthropicProvider, provider_trait::ProviderMessage}};
use crate::request_outputs::{ChatOutput};
use std::{sync::{Arc, Mutex}};
use crate::db::Store;
use crate::services::provider_trait::{ChatRequest, LLMProvider } ;
use std::time::{SystemTime, UNIX_EPOCH};
use poem::{
    Error,
    http::StatusCode,
};


#[handler]
pub async fn chat(Json(data):Json<ChatInput>,Data(s):Data<&Arc<Mutex<Store>>>) -> Result<Json<ChatOutput>, Error>  {
    let provider = data.provder;
    let provider_api_key = data.provder_api_key;
    let prompt = data.message;

   match provider.as_str() {
       "anthropic" => {
            let message = ProviderMessage {
                content: prompt,
                role:"user".to_string()
            };

            let request = ChatRequest { 
                model: data.model.to_string(), 
                messages:vec![message],
                temperature:Some(0.0),
                max_tokens:Some(100)
            };

            let created: i64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as i64;
            let provider  = AnthropicProvider::new();
            let result = provider.chat_completion(&provider_api_key, request).await.map_err(|_| Error::from_status(StatusCode::BAD_GATEWAY))?;
            let choice = Choice {
                message:Message {
                    role:"assistant".to_string(),
                    content:result.content
                }
            };

            let response = ChatOutput {
                model:result.model,
                choices:vec![choice],
                created
            };
            Ok(Json(response))
       } 
       _=> Err(Error::from_status(StatusCode::UNAUTHORIZED)),
   }
}