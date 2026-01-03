use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserOuput {
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageResponse {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ChatOutput {
    pub created: i64,
    pub model: String,
    pub stop_reason:String,
    pub choices: Vec<Choice>,
    pub usage: UsageResponse
}


#[derive(Serialize, Deserialize)]
pub struct SigninOutput {
    pub jwt: String,
}