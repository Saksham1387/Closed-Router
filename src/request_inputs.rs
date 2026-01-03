use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatInput {
    pub api_key:String,
    pub provider_api_key:String,
    pub provder:String,
    pub message:String,
    pub model:String
}

#[derive(Serialize, Deserialize)]
pub struct SigninInput {
    pub email: String,
    pub password: String,
}