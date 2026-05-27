use crate::config::schema;

use chrono::NaiveDate;
use diesel::{Selectable, deserialize::Queryable, prelude::*};
use serde::{Deserialize, Serialize};

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

#[derive(Insertable, Clone, Deserialize)]
#[diesel(table_name = schema::experiences)]
pub struct NewExperience {
    pub title: String,
    pub company_name: String,
    pub description: String,
    pub location: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Insertable, AsChangeset, Clone, Deserialize)]
#[diesel(table_name = schema::experiences)]
pub struct UpdateExperience {
    pub title: Option<String>,
    pub company_name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<NaiveDate>,
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
