use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::jwt::JwtService;
use crate::presentation::dto::AuthenticatedUser;

pub async fn authenticate_user(
  token: &str,
  jwt_service: &JwtService,
  auth_service: &AuthService<PostgresUserRepository>,
) -> Option<AuthenticatedUser> {
  let claims = jwt_service.verify_token(token).ok()?;
  let user = auth_service.get(claims.user_id).await.ok()?;

  Some(AuthenticatedUser {
    user_id: user.id,
    username: user.username,
    email: user.email,
  })
}
