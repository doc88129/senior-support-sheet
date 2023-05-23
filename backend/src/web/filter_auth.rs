use std::sync::Arc;

use surrealdb::opt::auth::Jwt;
use warp::{Filter, Rejection};

use crate::{model::database::Database, security::do_auth};

use super::{filter_utils::with_db, Error};

const HEADER_XAUTH: &str = "x-auth";

pub fn with_token(db: Arc<Database>) -> impl Filter<Extract = (Jwt,), Error = Rejection> + Clone {
	warp::any()
		.and(with_db(db))
		.and(warp::header::optional(HEADER_XAUTH))
		.and_then(|db: Arc<Database>, xauth: Option<String>| async move {
			match xauth {
				Some(xauth) => {
					let token = do_auth(&db, Jwt::from(xauth)).await?;
					Ok::<Jwt, Rejection>(token)
				}
				None => Err(Error::FailedAuthMissingToken.into()),
			}
		})
}
