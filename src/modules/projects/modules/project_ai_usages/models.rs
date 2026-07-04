use crate::config::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::project_ai_usages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectAiUsage {
    pub id: i32,
    pub title: String,
    pub val: i16,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug, Clone, Deserialize)]
#[diesel(table_name = schema::project_ai_usages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectAiUsage {
    pub title: String,
    pub val: i16,
    pub description: Option<String>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = schema::project_ai_usages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectAiUsage {
    pub title: Option<String>,
    pub val: Option<i16>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}
