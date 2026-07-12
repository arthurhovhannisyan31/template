use axum::{Router, response::Json, routing::get};
use chrono::Utc;
use serde_json::{Value, json};

pub fn get_health_router() -> Router {
  Router::new().route("/health", get(health))
}

async fn health() -> Json<Value> {
  Json(json!({
    "status": "ok",
    "timestamp": Utc::now(),
  }))
}
