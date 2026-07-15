use crate::infrastructure::constants::TOKEN_EXPIRATION_HOURS;

use axum_extra::extract::cookie::{Cookie, SameSite};

pub fn get_auth_cookie(token: &str, is_secure: bool) -> Cookie<'static> {
  let mut cookie = Cookie::new("Authorization", format!("Bearer {token}"));
  cookie.set_path("/");
  cookie.set_http_only(true);
  cookie.set_secure(is_secure);
  cookie.set_same_site(SameSite::Strict);
  cookie.set_max_age(cookie::time::Duration::hours(TOKEN_EXPIRATION_HOURS));

  cookie
}
