use axum::extract::FromRef;
use std::sync::Arc;

use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::jwt::JwtService;

#[derive(Clone)]
pub struct AppState {
  pub auth_state: Arc<AuthState>,
  pub app_config: Arc<AppConfig>,
}

#[derive(Clone)]
pub struct AuthState {
  pub auth_service: AuthService<PostgresUserRepository>,
  pub jwt_service: JwtService,
}

impl FromRef<AppState> for Arc<AuthState> {
  fn from_ref(app_state: &AppState) -> Self {
    app_state.auth_state.clone()
  }
}

impl FromRef<AppState> for Arc<AppConfig> {
  fn from_ref(app_state: &AppState) -> Self {
    app_state.app_config.clone()
  }
}
