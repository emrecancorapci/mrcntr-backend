use crate::{config::schema, modules::projects::Project};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Selectable, Identifiable, Associations, Debug, Clone, Serialize, Deserialize,
)]
#[diesel(table_name = schema::project_links)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectLink {
    pub id: i32,
    pub sort_order: i16,
    pub title: String,
    pub link: String,
    pub project_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone, Deserialize)]
#[diesel(table_name = schema::project_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectLink {
    pub sort_order: i16,
    pub title: String,
    pub link: String,
    pub project_id: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = schema::project_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectLink {
    pub sort_order: Option<i16>,
    pub title: Option<String>,
    pub link: Option<String>,
}
