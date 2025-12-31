
use poem::{Route, Server, get, listener::TcpListener};
use poem::{
    handler
};

use crate::services::anthropic::AnthropicProvider;
use crate::services::provider_trait::{ChatRequest, LLMProvider, Message};

pub mod schema;
pub mod config;
pub mod db;
pub mod services;
pub mod models;

#[handler]
fn some_thing() -> String{
    return String::from("Hello");
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {


    let message = Message {
        content: "Who are you ?".to_string(),
        role:"user".to_string()
    };

    let request = ChatRequest { 
        model: "claude-sonnet-4-5-20250".to_string(), 
        messages:vec![message],
        temperature:Some(0.0),
        max_tokens:Some(100)
    };

    let anthropic = AnthropicProvider::new();
    let result = anthropic.chat_completion("anthropic key", request).await;
    println!("{:?}",result);
    let app = Route::new().at("/",get(some_thing));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
    .name("Hello")
    .run(app)
    .await
}
