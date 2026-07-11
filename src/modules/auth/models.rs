use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::modules::users::{PASS_MAX_LEN, PASS_MIN_LEN};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uuid: String,
    pub exp: usize,
}

#[derive(Validate, ToSchema, Debug, Deserialize)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = (PASS_MIN_LEN as u64), max = (PASS_MAX_LEN as u64)))]
    pub password: String,
}
#[derive(ToSchema, Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub enum AppRoles {
    Admin,
    Author,
    User,
}

pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_AUTHOR: &str = "author";
pub const ROLE_USER: &str = "user";

impl From<AppRoles> for String {
    fn from(val: AppRoles) -> Self {
        val.to_string()
    }
}

impl From<AppRoles> for &str {
    fn from(val: AppRoles) -> Self {
        match val {
            AppRoles::Admin => ROLE_ADMIN,
            AppRoles::Author => ROLE_AUTHOR,
            AppRoles::User => ROLE_USER,
        }
    }
}

impl ToString for AppRoles {
    fn to_string(&self) -> String {
        match self {
            AppRoles::Admin => ROLE_ADMIN.to_string(),
            AppRoles::Author => ROLE_AUTHOR.to_string(),
            AppRoles::User => ROLE_USER.to_string(),
        }
    }
}
