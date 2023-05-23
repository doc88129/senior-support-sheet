use surrealdb::opt::auth::Jwt;
use thiserror::Error as ThisError;

use crate::model::database::Database;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    SurrealDBAuthError(#[from] surrealdb::Error),
}

pub async fn do_auth(database: &Database, token: Jwt) -> Result<Jwt, Error> {
    database.authenticate(token.clone()).await?;
    Ok(token)
}