use std::{sync::Arc, path::Path, convert::Infallible};
use serde_json::json;
use warp::Filter;

use crate::{model::{database::Database, self}, security, web::user::user_rest_filters};

mod user;
mod filter_utils;
mod filter_auth;

pub async fn start_web(web_folder: &str, web_port: u16, database: Arc<Database>) -> Result<(), Error> {

    if !Path::new(web_folder).exists() {
        return Err(Error::WebFolderDoesNotExist(web_folder.to_string()));
    }

    let apis = user_rest_filters("api", database);

    let content = warp::fs::dir(web_folder.to_string());
    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", web_folder)));
    let static_site = content.or(root_index);

    let routes = apis.or(static_site).recover(handle_rejection);

    println!("Starting web server on port {} at {}", web_port, web_folder);
    warp::serve(routes).run(([127, 0, 0, 1], web_port)).await;

    Ok(())
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    println!("ERROR - {:?}", err);
    let user_mesage = match err.find::<WebErrorMessage>() {
        Some(err) => err.typ.to_string(),
        None => "Unknown error".to_string(),
    };

    let result = json!({"errorMessage:" : user_mesage});
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(
        result,
        warp::http::StatusCode::BAD_REQUEST,
    ))
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' does not exist")]
    WebFolderDoesNotExist(String),

    #[error("Web server error due to missing token")]
    FailedAuthMissingToken,
}

#[derive(Debug)]
pub struct WebErrorMessage {
    pub typ: &'static str,
    pub message: String,
}

impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage { typ, message })
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(err: self::Error) -> Self {
        WebErrorMessage::rejection("WebError", format!("{:?}", err))
    }
}
impl From<model::Error> for warp::Rejection {
    fn from(err: model::Error) -> Self {
        WebErrorMessage::rejection("ModelError", format!("{:?}", err))
    }
}
impl From<security::Error> for warp::Rejection {
    fn from(err: security::Error) -> Self {
        WebErrorMessage::rejection("SecurityError", format!("{:?}", err))
    }
}