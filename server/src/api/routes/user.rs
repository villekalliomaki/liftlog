use crate::{api::response::Response, models::user::User};
use axum::{extract::State, Json};
use sqlx::PgPool;
use tracing::{error, info};

pub async fn new(State(pg_pool): State<PgPool>) -> Json<Response<i32>> {
    let new_user_result = User::new(&pg_pool, "username1".into(), "password1".into()).await;

    match new_user_result {
        Ok(value) => info!("Created new user: {:#?}", value),
        Err(error) => error!("Failed to create new user: {}", error),
    }

    Json(Response::data(1))
}
