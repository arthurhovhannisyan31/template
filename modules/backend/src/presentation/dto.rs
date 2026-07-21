use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

/// CreateUserRequest requirements
///
/// Username:
/// - Length 8-255 characters
/// - Only alfa-numeric characters
///
/// Email:
/// - length at most 255 characters
/// - should match `email` regex
///
/// Password:
/// - length 8-100 characters
/// - Include 1 uppercase letter: A-Z
/// - Include 1 lowercase letter: a-z
/// - Include 1 numeric character: 0-9
/// - Include a special character: !,$,#,%, etc

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
  #[validate(
    length(min = 8, max = 255),
    custom(function = "validate_alfa_numeric_username")
  )]
  pub username: String,
  #[validate(length(max = 255), email)]
  pub email: String,
  #[validate(
    length(min = 8, max = 100),
    custom(function = "validate_password_strength")
  )]
  pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct AuthenticatedUser {
  pub email: String,
  pub user_id: String,
  pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AuthResponse {
  pub token: String,
  pub user: AuthenticatedUser,
}

fn validate_alfa_numeric_username(
  username: &str,
) -> Result<(), ValidationError> {
  if username.is_empty() {
    return Err(ValidationError::new("Username cannot be empty"));
  }

  if !username.chars().all(char::is_alphanumeric) {
    return Err(ValidationError::new(
      "Username should include only alphanumeric values",
    ));
  }

  Ok(())
}

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
  let mut password_chars = password.chars();

  if !password_chars.any(char::is_uppercase) {
    return Err(ValidationError::new(
      "Password should include at least 1 uppercase character",
    ));
  }
  if !password_chars.any(char::is_lowercase) {
    return Err(ValidationError::new(
      "Password should include at least 1 lowercase character",
    ));
  }
  if !password_chars.any(char::is_numeric) {
    return Err(ValidationError::new(
      "Password should Include at least 1 numeric character",
    ));
  }
  if !password_chars.any(|char| char::is_ascii_punctuation(&char)) {
    return Err(ValidationError::new(
      "Password should Include at least 1 numeric character",
    ));
  }

  Ok(())
}

// TOOD Prop tests for CreateUserRequest
#[cfg(test)]
mod test_create_user_request {
  #[test]
  fn test() {
    todo!()
  }
}
