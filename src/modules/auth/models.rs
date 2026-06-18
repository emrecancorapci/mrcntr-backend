use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uuid: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
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
