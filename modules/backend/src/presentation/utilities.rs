use crate::infrastructure::openapi::OpenApiSpec;

use axum::{Router, http::StatusCode, response::Json, routing::get};
use chrono::Utc;
use serde_json::{Value, json};
use utoipa::OpenApi;

pub fn get_utilities_router() -> Router {
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
async fn openapi() -> Result<String, StatusCode> {
  match OpenApiSpec::openapi().to_json() {
    Ok(res) => Ok(res),
    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
  }
}
