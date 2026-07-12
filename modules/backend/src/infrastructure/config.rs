use crate::infrastructure::error::ServerError;

use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
  pub host: String,
  pub http_port: u16,
  pub database_url: String,
  pub jwt_secret: String,
  pub cors_origins: Vec<String>,
}

impl AppConfig {
  pub fn from_env() -> Result<Self, ServerError> {
    let docker_container = env::var("DOCKER_CONTAINER")
      .unwrap_or("false".to_owned())
      .eq("false");
    // Load variables when run locally
    if docker_container {
      dotenvy::dotenv()?;
    }

    let host = env::var("BACKEND_HOST").unwrap_or_else(|_| "localhost".into());
    let http_port = env::var("BACKEND_HTTP_PORT")
      .unwrap_or_else(|_| "8080".into())
      .parse()
      .map_err(|e| {
        ServerError::VarError(format!("Invalid SERVER_HTTP_PORT variable: {e}"))
      })?;
    let database_url = env::var("DATABASE_URL").map_err(|e| {
      ServerError::VarError(format!("Missing DATABASE_URL: {e}"))
    })?;
    let jwt_secret = env::var("BACKEND_JWT_SECRET").map_err(|e| {
      ServerError::VarError(format!("Missing BACKEND_JWT_SECRET: {e}"))
    })?;
    let cors_origins = env::var("BACKEND_CORS_ORIGINS")
      .map_err(|e| {
        ServerError::VarError(format!("Missing BACKEND_CORS_ORIGINS: {e}"))
      })?
      .split(',')
      .map(|s| s.trim().to_string())
      .filter(|s| !s.is_empty())
      .collect();

    Ok(Self {
      host,
      http_port,
      database_url,
      jwt_secret,
      cors_origins,
    })
  }
}
