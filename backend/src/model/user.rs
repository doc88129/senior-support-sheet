use serde_derive::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
use surrealdb::{Response, opt::auth::Jwt};
use super::{Error, database::Database};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub pid: i64,
    pub name: String,
    pub rank: SupportTeamRank,
}

#[derive(Debug, EnumString, EnumIter, Serialize, Deserialize)]
pub enum SupportTeamRank {
    NoWhiteList,
    SupportTeam1,
    SupportTeam2,
    SupportTeam3,
    SupportTeam4,
    SeniorSupportTeam,
    LeadSupportTeam,
}

pub struct UserMac;

impl UserMac {
    pub async fn create(database: &Database, token: Jwt, data: &User) -> Result<User, Error> {
        database.authenticate(token).await?;
        let user = database
            .create(("user", data.pid.to_string()))
            .content(data)
            .await?;
        Ok(user)
    }

    pub async fn fetch_one(database: &Database, token: Jwt, pid: i64) -> Result<User, Error> {
        database.authenticate(token).await?;
        let user = database
            .select(("user", pid.to_string()))
            .await?;
        Ok(user)
    }

    pub async fn fetch_all(database: &Database, token: Jwt) -> Result<Vec<User>, Error> {
        database.authenticate(token).await?;
        let users = database.select("user").await?;
        Ok(users)
    }

    pub async fn update_rank(
        database: &Database,
        token: Jwt,
        new_rank: &SupportTeamRank, 
        pid: i64
    ) -> Result<Response, Error> {
        database.authenticate(token).await?;
        let user = database
            .query("UPDATE user:$pid SET rank = $rank")
            .bind(("pid", pid))
            .bind(("rank", new_rank))
            .await?;
        Ok(user)
    }

    pub async fn update_name(
        database: &Database, 
        token: Jwt,
        new_name: &str, 
        pid: i64
    ) -> Result<Response, Error> {
        database.authenticate(token).await?;
        let user = database
            .query("UPDATE user:$pid SET name = $name")
            .bind(("pid", pid))
            .bind(("name", new_name))
            .await?;
        Ok(user)
    }

    pub async fn remove(
        database: &Database, 
        token: Jwt,
        pid: i64
    ) -> Result<User, Error> {
        database.authenticate(token).await?;
        let user = database
            .delete(("user", pid.to_string()))
            .await?;
        Ok(user)
    }
}
