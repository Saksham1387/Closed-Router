use poem::{handler, web::{Data, Json}};
use crate::{db::Store, request_inputs::ChatInput, request_outputs::{ChatOutput, Choice, Message, UsageResponse}, services::{anthropic::AnthropicProvider, openai::OpenaiProvider, provider_trait::ProviderMessage}};
use crate::services::provider_trait::{ChatRequest, LLMProvider } ;
use std::{sync::{Arc,Mutex}, time::{SystemTime, UNIX_EPOCH}};
use crate::models::logs::LogCreateInput;
use poem::{
    Error,
    http::StatusCode,
};
use std::convert::TryFrom;

#[handler]
pub async fn chat(Json(data):Json<ChatInput>,Data(s):Data<&Arc<Mutex<Store>>>) -> Result<Json<ChatOutput>, Error>  {
    let provider = data.provder;
    let provider_api_key = data.provider_api_key;
    let prompt = data.message;
    let user_api_key = data.api_key;
    let mut user_id:String = String::from(""); 

    {
        let mut locked_s = s.lock().unwrap();

        let (id, is_verified) = locked_s.verify_api_key(user_api_key);

        match id {
            Some(id) => user_id = id,
            None => return Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }

        if !is_verified {
        return Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }
    }

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
                stop_reason:result.stop_reason,
                choices:vec![choice],
                created,
                usage: UsageResponse {
                    prompt_tokens: result.usage.prompt_tokens,
                    completion_tokens: result.usage.completion_tokens,
                    total_tokens: result.usage.total_tokens
                }
            };

            let mut locked_s = s.lock().unwrap();
            locked_s.create_log(LogCreateInput {
                model:response.model.clone(),
                prompt_tokens:i32::try_from(response.usage.prompt_tokens).ok(),
                completion_tokens:i32::try_from(response.usage.completion_tokens).ok(),
                total_tokens:i32::try_from(response.usage.total_tokens).ok(),
                status_code:200,
                user_id:user_id,
                error_messages_input:Some(String::from("No errror"))
            });
            
            Ok(Json(response))
       } 

       "openai" => {
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
            let provider = OpenaiProvider::new();
            let result = provider.chat_completion(&provider_api_key, request).await.map_err(|_| Error::from_status(StatusCode::BAD_GATEWAY))?;

            let choice = Choice {
                message:Message {
                    role:"assistant".to_string(),
                    content:result.content
                }
            };

            let response = ChatOutput {
                model:result.model,
                stop_reason:result.stop_reason,
                choices:vec![choice],
                created,
                usage: UsageResponse {
                    prompt_tokens: result.usage.prompt_tokens,
                    completion_tokens: result.usage.completion_tokens,
                    total_tokens: result.usage.total_tokens
                }
            };
            Ok(Json(response))
       }
       _=> Err(Error::from_status(StatusCode::UNAUTHORIZED)),
   }
}