#[cfg(test)]
mod server_test {
  use axum::Router;
  use axum::http::StatusCode;
  use axum_test::{TestServer, expect_json};
  use backend::application::auth_service::AuthService;
  use backend::data::user_repository::PostgresUserRepository;
  use backend::infrastructure::{
    config::AppConfig, database::create_pool, error::ServerError,
    jwt::JwtService,
  };
  use backend::presentation::{common::AuthState, init::build_router};
  use serde_json::json;
  use std::sync::Arc;

  async fn setup_router() -> Result<Router, ServerError> {
    let app_config = AppConfig::from_env()?;
    let pool = create_pool(&app_config.database_url).await?;

    let jwt_service = JwtService::new(app_config.jwt_secret.clone());
    let users_repo = PostgresUserRepository::new(pool.clone());
    let auth_service = AuthService::new(users_repo, jwt_service.clone());
    let auth_state = Arc::new(AuthState {
      auth_service,
      jwt_service,
    });

    Ok(build_router(auth_state, &app_config))
  }

  #[tokio::test]
  async fn test_health_route() -> Result<(), ServerError> {
    let router = setup_router().await?;
    let server = TestServer::new(router);

    let response = server.get("/api/health").expect_success().await;

    assert_eq!(response.status_code(), StatusCode::OK);
    response.assert_json(&json!({
      "status": expect_json::string(),
      "timestamp": expect_json::string(),
    }));

    Ok(())
  }
}
