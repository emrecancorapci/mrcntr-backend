use crate::{config::schema, modules::projects::Project};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(
    Queryable, Selectable, Identifiable, Associations, ToSchema, Serialize, Deserialize, Clone,
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

#[derive(Insertable, ToSchema, Deserialize)]
#[diesel(table_name = schema::project_blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectBlock {
    pub sort_order: i16,
    pub title: String,
    pub content: String,
    pub project_id: i32,
    pub is_active: bool,
}

#[derive(ToSchema, Validate, Deserialize, Clone)]
pub struct NewProjectBlockRequest {
    #[validate(length(min = 3))]
    pub title: String,
    #[validate(length(min = 3))]
    pub content: String,
    pub sort_order: i16,
    pub is_active: bool,
}

impl NewProjectBlock {
    pub fn from_request(value: NewProjectBlockRequest, project_id: i32) -> Self {
        Self {
            sort_order: value.sort_order,
            title: value.title,
            content: value.content,
            is_active: value.is_active,
            project_id,
        }
    }
}

#[derive(AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = schema::project_blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectBlock {
    pub sort_order: Option<i16>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_active: Option<bool>,
}
