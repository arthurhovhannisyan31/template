use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct AuthenticatedUser {
  pub email: String,
  pub user_id: String,
  pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AuthResponse {
  pub token: String,
  pub user: AuthenticatedUser,
}
