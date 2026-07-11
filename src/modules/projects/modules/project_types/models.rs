use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, ToSchema, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::project_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectType {
    pub id: i32,
    pub title: String,
    pub sort_order: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, ToSchema, Clone, Deserialize)]
#[diesel(table_name = schema::project_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectType {
    pub title: String,
    pub sort_order: i16,
}

#[derive(AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = schema::project_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectType {
    pub title: Option<String>,
    pub sort_order: Option<i16>,
}
