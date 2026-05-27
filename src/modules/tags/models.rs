use crate::config::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub slug: String,
    pub title: String,
}

#[derive(AsChangeset, Debug, Clone, Deserialize)]
#[diesel(table_name = schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateTag {
    pub title: String,
}
