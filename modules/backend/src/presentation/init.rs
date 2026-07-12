use crate::infrastructure::config::AppConfig;
use crate::infrastructure::error::ServerError;
use crate::presentation::{
  auth::get_auth_router, common::AuthState, health::get_health_router,
  protected::get_protected_router,
};

use crate::infrastructure::cors::build_cors_layer;
use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::info;

pub async fn init_http_server(
  auth_state: Arc<AuthState>,
  app_config: &AppConfig,
) -> Result<(), ServerError> {
  let merged_router = Router::new()
    .merge(get_auth_router(auth_state.clone()))
    .merge(get_health_router())
    .merge(get_protected_router(auth_state.clone()));

  let root_router = Router::new()
    .nest("/api", merged_router)
    .layer(TraceLayer::new_for_http().on_request(
      |request: &axum::extract::Request<_>, _span: &tracing::Span| {
        info!("{:?}", request);
      },
    ))
    .layer(CompressionLayer::new())
    .layer(build_cors_layer(app_config));

  let listener =
    TcpListener::bind((app_config.host.to_string(), app_config.http_port))
      .await?;

  Ok(axum::serve(listener, root_router).await?)
}
