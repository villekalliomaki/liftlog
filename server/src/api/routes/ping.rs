use crate::api::response::{RouteResponse, RouteSuccess};
use axum::http::StatusCode;

pub async fn handle() -> RouteResponse<&'static str> {
    Ok(RouteSuccess::new("Pong.", "pong", StatusCode::OK))
}
