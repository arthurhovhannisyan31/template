use crate::infrastructure::auth::authenticate_user;
use crate::presentation::state::AppState;

use axum::{
  extract::{Request, State},
  http::{HeaderMap, StatusCode, header},
  middleware::Next,
  response::Response,
};

pub async fn auth(
  State(app_state): State<AppState>,
  headers: HeaderMap,
  mut request: Request,
  next: Next,
) -> Result<Response, StatusCode> {
  let token = get_token(&headers).ok_or(StatusCode::UNAUTHORIZED)?;
  let authenticated_user = authenticate_user(
    token,
    &app_state.auth_state.jwt_service,
    &app_state.auth_state.auth_service,
  )
  .await
  .ok_or(StatusCode::UNAUTHORIZED)?;

  request.extensions_mut().insert(authenticated_user);
  Ok(next.run(request).await)
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
  let auth_header = headers
    .get(header::AUTHORIZATION)
    .and_then(|value| value.to_str().ok())?;
  let (scheme, token) = auth_header.split_once(" ")?;
  (scheme.eq_ignore_ascii_case("bearer")).then_some(token)
}
