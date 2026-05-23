use crate::config::schema;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub slug: String,
    pub title: String,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = schema::tags)]
pub struct NewTag<'a> {
    pub slug: &'a str,
    pub title: &'a str,
}
