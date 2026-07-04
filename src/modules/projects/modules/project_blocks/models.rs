use crate::{config::schema, modules::projects::Project};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Clone, Serialize, Deserialize,
)]
#[diesel(table_name = schema::project_blocks)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectBlock {
    pub id: i32,
    pub sort_order: i16,
    pub title: String,
    pub content: String,
    pub is_active: bool,
    pub project_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug, Clone, Deserialize)]
#[diesel(table_name = schema::project_blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectBlock {
    pub sort_order: i16,
    pub title: String,
    pub content: String,
    pub project_id: i32,
    pub is_active: bool,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = schema::project_blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectBlock {
    pub sort_order: Option<i16>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_active: Option<bool>,
}
