use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub const PASS_MIN_LEN: u8 = 12;
pub const PASS_MAX_LEN: u8 = 128;

#[derive(Queryable, Selectable, Validate, Debug, Clone, Serialize, Deserialize)]
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

#[derive(Validate, Serialize, Deserialize)]
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

#[derive(Validate, Serialize, Deserialize)]
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

#[derive(Serialize, Clone)]
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
            role: None,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
