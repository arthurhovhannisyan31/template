use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthenticatedUser {
  pub email: String,
  pub user_id: i64,
  pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
  pub token: String,
  pub user: AuthenticatedUser,
}
