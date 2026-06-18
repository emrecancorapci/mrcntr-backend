use crate::config::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::project_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectType {
    pub id: i32,
    pub title: String,
    pub sort_order: i16,
}

#[derive(Insertable, Debug, Clone, Deserialize)]
#[diesel(table_name = schema::project_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectType {
    pub title: String,
    pub sort_order: i16,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = schema::project_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProjectType {
    pub title: Option<String>,
    pub sort_order: Option<i16>,
}
