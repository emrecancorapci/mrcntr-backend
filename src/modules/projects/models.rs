use super::modules::{
    project_ai_usages::ProjectAiUsage, project_blocks::ProjectBlock, project_links::ProjectLink,
    project_statuses::ProjectStatus, project_types::ProjectType,
};
use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub project_description: String,
    pub content: String,
    pub year_created_at: i16,
    pub latest_version: Option<String>,
    pub project_status_id: Option<i32>,
    pub project_type_id: Option<i32>,
    pub project_ai_usage_id: Option<i32>,
    pub is_featured: bool,
    pub is_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProject {
    pub title: String,
    pub project_description: String,
    pub content: String,
    pub year_created_at: i16,
    pub latest_version: Option<String>,
    pub project_status_id: Option<i32>,
    pub project_type_id: Option<i32>,
    pub project_ai_usage_id: Option<i32>,
    pub is_featured: bool,
    pub is_visible: bool,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProject {
    pub title: Option<String>,
    pub project_description: Option<String>,
    pub content: Option<String>,
    pub year_created_at: Option<i16>,
    pub latest_version: Option<String>,
    pub project_status_id: Option<i32>,
    pub project_type_id: Option<i32>,
    pub project_ai_usage_id: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_visible: Option<bool>,
}

#[derive(Serialize)]
pub struct MergedProject {
    pub id: i32,
    pub title: String,
    pub project_description: String,
    pub content: String,
    pub year_created_at: i16,
    pub latest_version: Option<String>,
    pub is_visible: bool,
    pub is_featured: bool,

    pub project_status_id: Option<i32>,
    pub project_type_id: Option<i32>,
    pub project_ai_usage_id: Option<i32>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub project_type: ProjectType,
    pub project_ai_usage: ProjectAiUsage,

    pub project_blocks: Vec<ProjectBlock>,
    pub project_links: Vec<ProjectLink>,
    pub project_status: ProjectStatus,
}
