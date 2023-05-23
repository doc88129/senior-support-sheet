use std::{sync::Arc, path::Path};
use warp::Filter;

use crate::model::{database::Database};


pub async fn start_web(web_folder: &str, web_port: u16, database: Arc<Database>) -> Result<(), Error> {

    if !Path::new(web_folder).exists() {
        return Err(Error::WebFolderDoesNotExist(web_folder.to_string()));
    }

    let content = warp::fs::dir(web_folder.to_string());
    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", web_folder)));
    let static_site = content.or(root_index);

    let routes = static_site;

    println!("Starting web server on port {} at {}", web_port, web_folder);
    warp::serve(routes).run(([127, 0, 0, 1], web_port)).await;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' does not exist")]
    WebFolderDoesNotExist(String),
}