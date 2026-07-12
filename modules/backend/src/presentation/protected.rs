use axum::{Extension, Json, Router, middleware, routing::get};
use chrono::Utc;
use serde_json::{Value, json};
use std::sync::Arc;

use crate::presentation::{
  common::AuthState, dto::AuthenticatedUser, middleware::auth,
};

pub fn get_protected_router(auth_state: Arc<AuthState>) -> Router {
  Router::new()
    .route("/protected", get(protected))
    .layer(middleware::from_fn_with_state(auth_state, auth))
}

async fn protected(
  Extension(authenticated_user): Extension<AuthenticatedUser>,
) -> Json<Value> {
  Json(json!({
    "status": "ok",
    "timestamp": Utc::now(),
    "authenticated_user": authenticated_user,
  }))
}
