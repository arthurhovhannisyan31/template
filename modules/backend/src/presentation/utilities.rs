use crate::application::error::ApplicationError;
use crate::infrastructure::openapi::OpenApiSpec;
use crate::presentation::constants::routes;
use crate::presentation::state::AppState;

use axum::{Router, response::Json, routing::get};
use chrono::Utc;
use serde_json::{Value, json};
use utoipa::OpenApi;

pub fn get_utilities_router() -> Router<AppState> {
  Router::new()
    .route(routes::HEALTH, get(health))
    .route(routes::OPENAPI, get(openapi))
}

#[utoipa::path(
  get,
  path = routes::HEALTH,
  responses((status = OK, body = Value))
)]
async fn health() -> Json<Value> {
  Json(json!({
    "status": "ok",
    "timestamp": Utc::now(),
  }))
}

#[utoipa::path(
  get,
  path = routes::OPENAPI,
  responses((status = OK, body = Value))
)]
async fn openapi() -> Result<String, ApplicationError> {
  match OpenApiSpec::openapi().to_json() {
    Ok(res) => Ok(res),
    Err(err) => Err(ApplicationError::Internal(err.to_string())),
  }
}
