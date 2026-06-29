use crate::{config::schema, modules::projects::Project};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(
    Queryable, Selectable, Validate, Identifiable, Associations, Debug, Clone, Serialize, Deserialize,
)]
#[diesel(table_name = schema::project_links)]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectLink {
    pub id: i32,
    pub sort_order: i16,
    #[validate(length(min = 3))]
    pub title: String,
    #[validate(url)]
    pub link: String,
    pub project_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Validate, Debug, Clone, Deserialize)]
#[diesel(table_name = schema::project_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectLink {
    pub sort_order: i16,
    #[validate(length(min = 3))]
    pub title: String,
    #[validate(url)]
    pub link: String,
    pub project_id: i32,
}

#[derive(AsChangeset, Validate, Deserialize)]
#[diesel(table_name = schema::project_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectLink {
    pub sort_order: Option<i16>,
    #[validate(length(min = 3))]
    pub title: Option<String>,
    #[validate(url)]
    pub link: Option<String>,
}
