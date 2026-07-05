use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Selectable, Validate, Serialize, Deserialize)]
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

#[derive(Insertable, Validate, Deserialize)]
#[diesel(table_name = schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCategory {
    #[validate(length(min = 3))]
    pub slug: String,
    #[validate(length(min = 3))]
    pub title: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateCategory {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Validate, Deserialize)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 3))]
    pub slug: Option<String>,
    #[validate(length(min = 3))]
    pub title: Option<String>,
}

#[derive(Serialize)]
pub struct CategoryResponse {
    pub slug: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Category> for CategoryResponse {
    fn from(value: Category) -> Self {
        CategoryResponse {
            slug: value.slug,
            title: value.title,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
