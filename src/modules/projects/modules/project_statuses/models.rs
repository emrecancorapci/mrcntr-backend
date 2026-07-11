use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, ToSchema, Serialize, Deserialize, Clone)]
#[diesel(table_name = schema::project_statuses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectStatus {
    pub id: i32,
    pub title: String,
    pub sort_value: Option<i16>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, ToSchema, Deserialize, Clone)]
#[diesel(table_name = schema::project_statuses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectStatus {
    pub title: String,
    pub sort_value: Option<i16>,
}

#[derive(AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = schema::project_statuses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectStatus {
    pub title: Option<String>,
    pub sort_value: Option<i16>,
}
