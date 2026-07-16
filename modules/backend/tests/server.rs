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
  use backend::presentation::constants::routes;
  use backend::presentation::dto::{AuthResponse, CreateUserRequest};
  use backend::presentation::state::AuthState;
  use backend::presentation::{init::build_router, state::AppState};
  use serde_json::json;
  use std::sync::Arc;
  use uuid::{Uuid, Version};

  async fn setup_router() -> Result<Router, ServerError> {
    let app_config = AppConfig::from_env()?;
    let pool = create_pool(&app_config.database_url, 1).await?;

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

    Ok(build_router(app_state))
  }

  fn is_valid_v4_uuid(input: &str) -> bool {
    match Uuid::parse_str(input) {
      Ok(parsed_uuid) => parsed_uuid.get_version() == Some(Version::Random),
      Err(_) => false,
    }
  }

  fn with_base_route(path: &str) -> String {
    format!("/api/{}", path.strip_prefix("/").unwrap())
  }

  #[ignore]
  #[tokio::test]
  async fn test_health_route() -> Result<(), ServerError> {
    let router = setup_router().await?;
    let server = TestServer::new(router);

    let response = server
      .get(&with_base_route(routes::HEALTH))
      .expect_success()
      .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    response.assert_json(&json!({
      "status": expect_json::string(),
      "timestamp": expect_json::string(),
    }));

    Ok(())
  }

  #[ignore]
  #[tokio::test]
  async fn test_openapi_route() -> Result<(), ServerError> {
    let router = setup_router().await?;
    let server = TestServer::new(router);

    let response = server
      .get(&with_base_route(routes::OPENAPI))
      .expect_success()
      .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    response.assert_json(&json!({
      "openapi": expect_json::string(),
      "info": expect_json::object(),
      "paths": expect_json::object(),
      "components": expect_json::object(),
    }));

    Ok(())
  }

  #[ignore]
  #[tokio::test]
  async fn test_register_route() -> Result<(), ServerError> {
    let router = setup_router().await?;
    let server = TestServer::new(router);

    let create_user_request = CreateUserRequest {
      email: "email".into(),
      username: "username".into(),
      password: "password".into(),
    };
    let response = server
      .post(&with_base_route(routes::REGISTER))
      .json(&json!(create_user_request))
      .expect_success()
      .await;
    let auth_response = response.json::<AuthResponse>();

    assert_eq!(response.status_code(), StatusCode::CREATED);
    assert_eq!(auth_response.user.username, create_user_request.username);
    assert_eq!(auth_response.user.email, create_user_request.email);
    assert!(is_valid_v4_uuid(&auth_response.user.user_id));

    Ok(())
  }
}
