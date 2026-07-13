use crate::presentation::{
  auth::{__path_login, __path_register},
  protected::__path_protected,
  utilities::{__path_health, __path_openapi},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(login, register, protected, health, openapi))]
pub struct OpenApiSpec;
