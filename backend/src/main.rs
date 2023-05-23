#![allow(dead_code)]
use std::sync::Arc;

use model::database::{new_database_connection, signin, Credentials};

mod model;
mod security;
mod web;

const WEB_FOLDER: &'static str = "web-folder/";
const WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let web_folder = args.pop().unwrap_or_else(|| WEB_FOLDER.to_string());
    let web_port = args
        .pop()
        .unwrap_or_else(|| WEB_PORT.to_string())
        .parse::<u16>()
        .unwrap_or(WEB_PORT);

    let database = new_database_connection()
        .await
        .expect("Cannot connect to database");
    let cred = Credentials {
        user: "johndoe",
        pass: "password123",
    };
    let token = signin(&database, cred).await.expect("Cannot signin");

    web::start_web(&web_folder, web_port, Arc::new(database)).await;
}
