use super::modules::{
    project_ai_usages::ProjectAiUsage, project_statuses::ProjectStatus, project_types::ProjectType,
};
use crate::{
    config::schema,
    modules::{
        project_blocks::{NewProjectBlockRequest, ProjectBlock},
        project_links::ProjectLink,
        tags::Tag,
    },
};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

const TITLE_MIN_LEN: u8 = 3;
const DESC_MIN_LEN: u8 = 3;
const CONTENT_MIN_LEN: u8 = 3;

#[derive(Queryable, Selectable, Validate, Identifiable, ToSchema, Serialize, Deserialize)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i32,
    #[validate(length(min = (TITLE_MIN_LEN as u64)))]
    pub title: String,
    #[validate(length(min = (DESC_MIN_LEN as u64)))]
    pub project_description: String,
    #[validate(length(min = (CONTENT_MIN_LEN as u64)))]
    pub content: String,
    #[validate(range(min = 2020))]
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

#[derive(Insertable)]
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

#[derive(AsChangeset, ToSchema, Validate, Deserialize)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProject {
    #[validate(length(min = (TITLE_MIN_LEN as u64)))]
    pub title: Option<String>,
    #[validate(length(min = (DESC_MIN_LEN as u64)))]
    pub project_description: Option<String>,
    #[validate(length(min = (CONTENT_MIN_LEN as u64)))]
    pub content: Option<String>,
    #[validate(range(min = 2020))]
    pub year_created_at: Option<i16>,
    pub latest_version: Option<String>,
    pub project_status_id: Option<i32>,
    pub project_type_id: Option<i32>,
    pub project_ai_usage_id: Option<i32>,
    pub is_featured: Option<bool>,
    pub is_visible: Option<bool>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, ToSchema)]
pub struct ProjectResponse {
    pub id: i32,
    pub title: String,
    pub project_description: String,
    pub content: String,
    pub year_created_at: i16,
    pub latest_version: Option<String>,
    pub is_visible: bool,
    pub is_featured: bool,

    // M-O Ids
    pub project_status_id: Option<i32>,
    pub project_type_id: Option<i32>,
    pub project_ai_usage_id: Option<i32>,

    // M-O Objects
    pub project_type: Option<ProjectType>,
    pub project_ai_usage: Option<ProjectAiUsage>,
    pub project_status: Option<ProjectStatus>,

    // O-M Objects
    pub project_blocks: Vec<ProjectBlock>,
    pub project_links: Vec<ProjectLink>,

    // M-M Objects
    pub tags: Vec<Tag>,

    // Dates
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Validate, ToSchema, Deserialize, Clone)]
pub struct NewProjectRequest {
    #[validate(length(min = (TITLE_MIN_LEN as u64)))]
    pub title: String,
    #[validate(length(min = (DESC_MIN_LEN as u64)))]
    pub project_description: String,
    #[validate(length(min = (CONTENT_MIN_LEN as u64)))]
    pub content: String,
    #[validate(range(min = 2020))]
    pub year_created_at: i16,
    pub latest_version: Option<String>,
    pub is_visible: bool,
    pub is_featured: bool,
    pub published_at: Option<DateTime<Utc>>,

    pub project_status_id: Option<i32>,
    pub project_type_id: Option<i32>,
    pub project_ai_usage_id: Option<i32>,

    #[validate(nested)]
    pub project_blocks: Vec<NewProjectBlockRequest>,
    #[validate(nested)]
    pub project_links: Vec<NewProjectLinkRequest>,
    pub tags: Vec<i32>,
}

#[derive(Validate, ToSchema, Deserialize, Clone)]
pub struct NewProjectLinkRequest {
    pub sort_order: i16,
    #[validate(length(min = 3))]
    pub title: String,
    #[validate(url)]
    pub link: String,
}

impl From<NewProjectRequest> for NewProject {
    fn from(val: NewProjectRequest) -> Self {
        NewProject {
            title: val.title,
            project_description: val.project_description,
            content: val.content,
            year_created_at: val.year_created_at,
            latest_version: val.latest_version,
            project_status_id: val.project_status_id,
            project_type_id: val.project_type_id,
            project_ai_usage_id: val.project_ai_usage_id,
            is_featured: val.is_featured,
            is_visible: val.is_visible,
            published_at: val.published_at,
        }
    }
}

impl From<Project> for ProjectResponse {
    fn from(value: Project) -> Self {
        ProjectResponse {
            id: value.id,
            title: value.title,
            project_description: value.project_description,
            content: value.content,
            year_created_at: value.year_created_at,
            latest_version: value.latest_version,
            is_visible: value.is_visible,
            is_featured: value.is_featured,
            project_status_id: value.project_status_id,
            project_type_id: value.project_type_id,
            project_ai_usage_id: value.project_ai_usage_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            published_at: value.published_at,
            project_type: None,
            project_ai_usage: None,
            project_status: None,
            project_blocks: Vec::new(),
            project_links: Vec::new(),
            tags: Vec::new(),
        }
    }
}

impl ProjectResponse {
    pub fn from_complete(
        project: Project,
        project_status: Option<ProjectStatus>,
        project_type: Option<ProjectType>,
        project_ai_usage: Option<ProjectAiUsage>,
        project_blocks: Vec<ProjectBlock>,
        project_links: Vec<ProjectLink>,
        tags: Vec<Tag>,
    ) -> Self {
        Self {
            id: project.id,
            title: project.title,
            project_description: project.project_description,
            content: project.content,
            year_created_at: project.year_created_at,
            latest_version: project.latest_version,
            is_visible: project.is_visible,
            is_featured: project.is_featured,
            project_status_id: project.project_status_id,
            project_type_id: project.project_type_id,
            project_ai_usage_id: project.project_ai_usage_id,
            created_at: project.created_at,
            updated_at: project.updated_at,
            published_at: project.published_at,
            project_type,
            project_ai_usage,
            project_status,
            project_blocks,
            project_links,
            tags,
        }
    }
}
