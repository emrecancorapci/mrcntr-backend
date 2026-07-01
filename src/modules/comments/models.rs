use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub author_uuid: Uuid,
    pub content: String,
    pub blogpost_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug, Clone, Deserialize)]
#[diesel(table_name = schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewComment {
    pub content: String,
    pub blogpost_id: i32,
    pub parent_comment_id: Option<i32>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateComment {
    pub content: String,
}
