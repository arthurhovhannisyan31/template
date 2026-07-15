use crate::application::error::ApplicationError;
use crate::presentation::{
  dto::{AuthRequest, AuthResponse, AuthenticatedUser, CreateUserRequest},
  state::AppState,
  state::AuthState,
};

use crate::domain::user::User;
use axum::{
  Json, Router,
  body::Body,
  extract::State,
  http::{Response, StatusCode},
  response::IntoResponse,
  routing::post,
};
use serde_json::json;
use std::sync::Arc;
use tracing::info;

pub fn get_auth_router() -> Router<AppState> {
  Router::new()
    .route("/auth/login", post(login))
    .route("/auth/register", post(register))
}

#[utoipa::path(
  post,
  path = "/auth/login",
  responses((status = OK, body = AuthResponse))
)]
pub async fn login(
  State(auth_state): State<Arc<AuthState>>,
  Json(payload): Json<AuthRequest>,
) -> Result<impl IntoResponse, ApplicationError> {
  let token = auth_state
    .auth_service
    .login(&payload.email, &payload.password)
    .await?;
  let user = auth_state.auth_service.get_by_email(&payload.email).await?;

  build_auth_response(token.clone(), user)
}

#[utoipa::path(
  post,
  path = "/auth/register",
  responses((status = OK, body = AuthResponse))
)]
async fn register(
  State(auth_state): State<Arc<AuthState>>,
  Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApplicationError> {
  let user = auth_state
    .auth_service
    .register(
      payload.email.clone(),
      payload.password.clone(),
      payload.username.clone(),
    )
    .await?;
  info!(user_id = %user.id, email = %user.email, username = %user.username, "user registered");

  let token = auth_state
    .jwt_service
    .generate_token(user.id, user.username.clone())
    .map_err(|err| ApplicationError::Internal(err.to_string()))?;

  build_auth_response(token.clone(), user)
}

fn build_auth_response(
  token: String,
  user: User,
) -> Result<impl IntoResponse, ApplicationError> {
  let authenticated_user = AuthenticatedUser {
    user_id: user.id,
    email: user.email,
    username: user.username,
  };
  let response_body = json!(AuthResponse {
    user: authenticated_user,
    token: token.clone(),
  })
  .to_string();

  Response::builder()
    .status(StatusCode::CREATED)
    .body(Body::from(response_body))
    .map_err(|err| ApplicationError::Internal(err.to_string()))
}
