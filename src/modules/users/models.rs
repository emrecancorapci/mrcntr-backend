use crate::config::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub uuid: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]

pub struct NewUserBody {
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserBody {
    pub email: Option<String>,
    pub password: Option<String>,
}