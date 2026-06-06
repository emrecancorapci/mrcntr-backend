use crate::config::schema;

use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ExperienceInsertBody {
    pub title: String,
    pub company_name: String,
    pub description: String,
    pub location: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tags: Option<Vec<i32>>,
}

impl ExperienceInsertBody {
    pub fn to_new_experience(&self) -> NewExperience {
        NewExperience {
            title: self.title.clone(),
            company_name: self.company_name.clone(),
            description: self.description.clone(),
            location: self.location.clone(),
            start_date: self.start_date,
            end_date: self.end_date,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Deserialize)]
pub struct ExperienceUpdateBody {
    pub title: Option<String>,
    pub company_name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tags: Option<Vec<i32>>,
}

impl ExperienceUpdateBody {
    pub fn to_update_experience(&self) -> UpdateExperience {
        UpdateExperience {
            title: self.title.clone(),
            company_name: self.company_name.clone(),
            description: self.description.clone(),
            location: self.location.clone(),
            start_date: self.start_date,
            end_date: self.end_date,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}