use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use validator::Validate;

pub const PASS_MIN_LEN: u8 = 12;
pub const PASS_MAX_LEN: u8 = 128;

#[derive(Queryable, Selectable, Validate, Debug, Clone, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = schema::users)]
#[diesel(belongs_to(Role))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub uuid: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub summary: Option<String>,
    pub image_url: Option<String>,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub role_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub summary: Option<String>,
    pub image_url: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub role_id: i32,
}

#[derive(Validate, Serialize, Deserialize, ToSchema)]
pub struct NewUserBody {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub summary: Option<String>,
    pub image_url: Option<String>,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = (PASS_MIN_LEN as u64), max = (PASS_MAX_LEN as u64)))]
    pub password: String,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub summary: Option<String>,
    pub image_url: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Validate, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserBody {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub summary: Option<String>,
    pub image_url: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = (PASS_MIN_LEN as u64), max = (PASS_MAX_LEN as u64)))]
    pub password: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct UserResponse {
    pub uuid: Uuid,
    pub email: String,
    pub role: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(val: User) -> Self {
        UserResponse {
            uuid: val.uuid,
            email: val.email.to_string(),
            role: match val.role_id {
                1 => Some("admin".to_string()),
                2 => Some("author".to_string()),
                _ => Some("user".to_string()),
            },
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

impl UpdateUser {
    pub fn from_body(value: UpdateUserBody, password_hash: Option<String>) -> Self {
        UpdateUser {
            first_name: value.first_name,
            last_name: value.last_name,
            summary: value.summary,
            image_url: value.image_url,
            updated_at: chrono::Utc::now(),
            deleted_at: value.deleted_at,
            email: value.email,
            password_hash,
        }
    }
}

impl NewUser {
    pub fn from_body(value: NewUserBody, password_hash: &str) -> Self {
        NewUser {
            first_name: value.first_name,
            last_name: value.last_name,
            summary: value.summary,
            image_url: value.image_url,
            email: value.email,
            password_hash: password_hash.to_string(),
            role_id: 3,
        }
    }
}
