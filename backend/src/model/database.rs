use serde_derive::Serialize;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::{Database as DatabaseAuth, Jwt, Root};
use super::Error;

static DATABASE_URL: &str = "127.0.0.1:8000";

pub type Database = Surreal<Client>;

#[derive(Debug, Serialize)]
pub struct Credentials<'a> {
    pub user: &'a str,
    pub pass: &'a str,
}

pub async fn new_database_connection() -> Result<Database, Error> {
    let connection = Surreal::new::<Ws>(DATABASE_URL).await?;

    connection.signin(Root {
        username: "root",
        password: "root",
    }).await?;

    connection.query(
        "DEFINE NAMESPACE test; 
        USE NS test;
        DEFINE DATABASE test; 
        USE DB test;
        DEFINE LOGIN johndoe ON DATABASE PASSWORD 'password123'"
    ).await?;

    Ok(connection)
}

pub async fn signin(database: &Database, cred: Credentials<'_>) -> Result<Jwt, Error> {
    let token = database.signin(DatabaseAuth {
        namespace: "test",
        database: "test",
        username: cred.user,
        password: cred.pass,
    }).await?;
    Ok(token)
}

// #[cfg(test)]
// #[path = "../_tests/model_database.rs"]
// mod test;
