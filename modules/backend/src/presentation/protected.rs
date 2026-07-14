use crate::presentation::state::AppState;
use crate::presentation::{dto::AuthenticatedUser, middleware::auth};

use axum::{Extension, Json, Router, middleware, routing::get};
use chrono::Utc;
use serde_json::{Value, json};

pub fn get_protected_router(app_state: AppState) -> Router<AppState> {
  Router::new()
    .route("/protected", get(protected))
    .layer(middleware::from_fn_with_state(app_state, auth))
}

#[utoipa::path(
  get,
  path = "/protected",
  responses((status = OK, body = Value))
)]
async fn protected(
  Extension(authenticated_user): Extension<AuthenticatedUser>,
) -> Json<Value> {
  Json(json!({
    "status": "ok",
    "timestamp": Utc::now(),
    "authenticated_user": authenticated_user,
  }))
}
