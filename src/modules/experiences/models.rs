use crate::config::schema;

use chrono::NaiveDate;
use diesel::{Selectable, deserialize::Queryable, prelude::*};
use serde::Serialize;

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = schema::experiences)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Experience {
    pub id: i32,
    pub title: String,
    pub company_name: String,
    pub description: String,
    pub location: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Insertable, AsChangeset, Clone, Copy)]
#[diesel(table_name = schema::experiences)]
pub struct NewExperience<'a> {
    pub title: &'a str,
    pub company_name: &'a str,
    pub description: &'a str,
    pub location: &'a str,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = schema::experience_tags)]
#[diesel(belongs_to(Experience))]
#[diesel(belongs_to(Tag))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExperienceTag {
    pub experience_id: i32,
    pub tag_slug: String,
}
