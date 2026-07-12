use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::jwt::JwtService;

#[derive(Clone)]
pub struct AuthState {
  pub auth_service: AuthService<PostgresUserRepository>,
  pub jwt_service: JwtService,
}
