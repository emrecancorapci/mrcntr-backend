use crate::{config::schema, modules::projects::Project};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(
    Queryable,
    Selectable,
    Validate,
    Identifiable,
    Associations,
    ToSchema,
    Serialize,
    Deserialize,
    Clone,
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
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, ToSchema, Clone, Deserialize)]
#[diesel(table_name = schema::project_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectLink {
    pub sort_order: i16,
    pub title: String,
    pub link: String,
    pub project_id: i32,
}

#[derive(AsChangeset, ToSchema, Validate, Deserialize)]
#[diesel(table_name = schema::project_links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectLink {
    pub sort_order: Option<i16>,
    #[validate(length(min = 3))]
    pub title: Option<String>,
    #[validate(url)]
    pub link: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema, Validate, Clone)]
pub struct NewProjectLinkRequest {
    pub sort_order: i16,
    #[validate(length(min = 3))]
    pub title: String,
    #[validate(url)]
    pub link: String,
}

impl NewProjectLink {
    pub fn from_request(value: NewProjectLinkRequest, project_id: i32) -> Self {
        Self {
            sort_order: value.sort_order,
            title: value.title,
            link: value.link,
            project_id,
        }
    }
}
