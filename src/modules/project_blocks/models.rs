use crate::config::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::project_blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectBlock {
    pub id: i32,
    pub sort_order: i16,
    pub title: String,
    pub content: String,
    pub project_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone, Deserialize)]
#[diesel(table_name = schema::project_blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectBlock {
    pub sort_order: i16,
    pub title: String,
    pub content: String,
    pub project_id: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = schema::project_blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectBlock {
    pub sort_order: Option<i16>,
    pub title: Option<String>,
    pub content: Option<String>,
}
