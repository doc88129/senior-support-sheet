use std::{sync::Arc, convert::Infallible};

use warp::Filter;

use crate::model::database::Database;


pub fn with_db(database: Arc<Database>) -> impl Filter<Extract = (Arc<Database>,), Error = Infallible> + Clone {
    warp::any().map(move || database.clone())
}
