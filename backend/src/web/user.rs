use std::sync::Arc;

use serde::Serialize;
use serde_json::json;
use surrealdb::opt::auth::Jwt;
use warp::{Filter, reply::Json};

use crate::model::user::{User, SupportTeamRank};
use crate::model::{database::Database, user::UserMac};

use super::filter_utils::with_db;
use super::filter_auth::with_token;


pub fn user_rest_filters(
    base_path: &'static str,
    database: Arc<Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_path = warp::path(base_path).and(warp::path("users"));
    let common = with_db(database.clone()).and(with_token(database.clone()));

    let list = user_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(user_fetch_all);

    let get = user_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(user_fetch_one);

    let create = user_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(user_create);

    let update_rank = user_path
        .and(warp::path("rank"))
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(user_update_rank);

    let update_name = user_path
        .and(warp::path("name"))
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(user_update_name);

    let remove = user_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(user_remove);

    list.or(get).or(create).or(update_rank).or(update_name).or(remove)
}

pub async fn user_create(database: Arc<Database>, token: Jwt, data: User) -> Result<Json, warp::Rejection> {
    let user = UserMac::create(&database, token, &data).await?;

    json_response(user)
}

pub async fn user_fetch_one(database: Arc<Database>, token: Jwt, pid: i64) -> Result<Json, warp::Rejection> {
    let user = UserMac::fetch_one(&database, token, pid).await?;

    json_response(user)
}

pub async fn user_fetch_all(database: Arc<Database>, token: Jwt) -> Result<Json, warp::Rejection> {
    let users = UserMac::fetch_all(&database, token).await?;

    json_response(users)
}

pub async fn user_update_rank(database: Arc<Database>, token: Jwt, new_rank: SupportTeamRank, pid: i64) -> Result<Json, warp::Rejection> {
    UserMac::update_rank(&database, token.clone(), &new_rank, pid).await?;

    user_fetch_one(database, token, pid).await
}

pub async fn user_update_name(database: Arc<Database>, token: Jwt, new_name: String, pid: i64) -> Result<Json, warp::Rejection> {
    UserMac::update_name(&database, token.clone(), &new_name, pid).await?;

    user_fetch_one(database, token, pid).await
}

pub async fn user_remove(database: Arc<Database>, token: Jwt, pid: i64) -> Result<Json, warp::Rejection> {
    UserMac::remove(&database, token, pid).await?;

    json_response(json!({"message": format!("User with pid {} was removed", pid)}))
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}
