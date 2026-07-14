use crate::application::error::ApplicationError;
use crate::infrastructure::openapi::OpenApiSpec;
use crate::presentation::state::AppState;

use axum::{Router, response::Json, routing::get};
use chrono::Utc;
use serde_json::{Value, json};
use utoipa::OpenApi;

pub fn get_utilities_router() -> Router<AppState> {
  Router::new()
    .route("/health", get(health))
    .route("/openapi", get(openapi))
}

#[utoipa::path(
  get,
  path = "/health",
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
  path = "/openapi",
  responses((status = OK, body = Value))
)]
async fn openapi() -> Result<String, ApplicationError> {
  match OpenApiSpec::openapi().to_json() {
    Ok(res) => Ok(res),
    Err(err) => Err(ApplicationError::Internal(err.to_string())),
  }
}
