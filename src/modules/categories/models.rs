use crate::config::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub slug: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateCategory {
    pub title: String,
}
