use thiserror::Error as ThisError;

pub mod database;
pub mod user;
pub mod note;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    SurrealDBError(#[from] surrealdb::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
