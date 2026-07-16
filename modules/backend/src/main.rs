mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use application::auth_service::AuthService;
use data::user_repository::PostgresUserRepository;
use infrastructure::{
  config::AppConfig,
  database::{create_pool, run_migrations},
  error::ServerError,
  jwt::JwtService,
  logging::init_logging,
};
use presentation::{init::init_http_server, state::AppState, state::AuthState};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
  init_logging();

  let app_config = AppConfig::from_env()?;
  let pool =
    create_pool(&app_config.database_url, app_config.db_max_connections)
      .await?;

  run_migrations(&pool).await?;

  let jwt_service = JwtService::new(app_config.jwt_secret.clone());
  let users_repo = PostgresUserRepository::new(pool.clone());
  let auth_service = AuthService::new(users_repo, jwt_service.clone());
  let app_state = AppState {
    auth_state: Arc::new(AuthState {
      auth_service,
      jwt_service,
    }),
    app_config: Arc::new(app_config),
  };

  init_http_server(app_state).await?;

  Ok(())
}
