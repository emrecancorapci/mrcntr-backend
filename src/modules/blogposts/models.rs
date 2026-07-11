use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, ToSchema, Serialize, Deserialize, Clone)]
#[diesel(table_name = schema::blogposts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Blogpost {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub author_uuid: Uuid,
    pub is_visible: bool,
    pub category_slug: Option<String>,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, ToSchema, Deserialize, Clone)]
#[diesel(table_name = schema::blogposts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewBlogpost {
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub author_uuid: Uuid,
    pub is_visible: bool,
}

#[derive(AsChangeset, ToSchema, Deserialize)]
#[diesel(table_name = schema::blogposts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateBlogpost {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub is_visible: Option<bool>,
}
