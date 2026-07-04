use crate::config::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Insertable, Queryable, Validate, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    #[validate(length(min = 3))]
    pub slug: String,
    #[validate(length(min = 3))]
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct UpdateCategory {
    pub title: String,
}
