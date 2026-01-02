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


#[derive(Serialize, Deserialize)]
pub struct ChatOutput {
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,

}


#[derive(Serialize, Deserialize)]
pub struct SigninOutput {
    pub jwt: String,
}