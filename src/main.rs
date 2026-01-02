
use std::sync::{Arc,Mutex};
use poem::{EndpointExt, Route, Server, listener::TcpListener, post};

use poem::{
    handler
};
use crate::db::Store;
use crate::routes::user::{create_user,signin};
use crate::routes::chat::chat;
pub mod schema;
pub mod config;
pub mod db;
pub mod services;
pub mod models; 
pub mod request_inputs;
pub mod request_outputs;
pub mod routes;

#[handler]
fn some_thing() -> String{
    return String::from("Hello");
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app = Route::new()
        .at("/signup",post(create_user))
        .at("/signin", post(signin))
        .at("/chat/completion",post(chat))
        .data(s);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
    .name("Closed Router")
    .run(app)
    .await
}
