mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Arc;

use application::auth_service::AuthService;
use data::user_repository::PostgresUserRepository;
use infrastructure::{
  config::AppConfig,
  database::{create_pool, run_migrations},
  error::ServerError,
  jwt::JwtService,
  logging::init_logging,
};
use presentation::{common::AuthState, init::init_http_server};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
  init_logging();

  let config = AppConfig::from_env()?;
  let pool = create_pool(&config.database_url).await?;

  run_migrations(&pool).await?;

  let jwt_service = JwtService::new(config.jwt_secret.clone());
  let users_repo = PostgresUserRepository::new(pool.clone());
  let auth_service = AuthService::new(users_repo, jwt_service.clone());
  let auth_state = Arc::new(AuthState {
    auth_service,
    jwt_service,
  });

  init_http_server(auth_state.clone(), &config).await?;

  Ok(())
}
