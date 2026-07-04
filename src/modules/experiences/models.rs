use crate::{
    config::schema,
    modules::{experiences::experiences_tags::ExperienceTag, tags::Tag},
};

use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Identifiable, Selectable, Validate, Serialize, PartialEq, Debug, Clone)]
#[diesel(table_name = schema::experiences)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Experience {
    pub id: i32,
    #[validate(length(min = 3))]
    pub title: String,
    #[validate(length(min = 3))]
    pub company_name: String,
    #[validate(length(min = 3))]
    pub description: String,
    pub location: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Validate, Clone, Deserialize)]
#[diesel(table_name = schema::experiences)]
pub struct NewExperience {
    #[validate(length(min = 3))]
    pub title: String,
    #[validate(length(min = 3))]
    pub company_name: String,
    #[validate(length(min = 3))]
    pub description: String,
    pub location: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
}

#[derive(AsChangeset, Validate, Clone, Deserialize)]
#[diesel(table_name = schema::experiences)]
pub struct UpdateExperience {
    #[validate(length(min = 3))]
    pub title: Option<String>,
    #[validate(length(min = 3))]
    pub company_name: Option<String>,
    #[validate(length(min = 3))]
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: Option<bool>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Validate, Deserialize)]
pub struct ExperienceInsertBody {
    #[validate(length(min = 3))]
    pub title: String,
    #[validate(length(min = 3))]
    pub company_name: String,
    #[validate(length(min = 3))]
    pub description: String,
    pub location: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
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
            is_active: self.is_active,
        }
    }
}

#[derive(Validate, Deserialize)]
pub struct ExperienceUpdateBody {
    #[validate(length(min = 3))]
    pub title: Option<String>,
    #[validate(length(min = 3))]
    pub company_name: Option<String>,
    #[validate(length(min = 3))]
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: Option<bool>,
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
            is_active: self.is_active,
            updated_at: Utc::now(),
        }
    }
}

#[derive(Serialize)]
pub struct ExperienceResponse {
    pub id: i32,
    pub title: String,
    pub company_name: String,
    pub description: String,
    pub location: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<Tag>,
}

impl From<Experience> for ExperienceResponse {
    fn from(value: Experience) -> Self {
        ExperienceResponse {
            id: value.id,
            title: value.title,
            company_name: value.company_name,
            description: value.description,
            location: value.location,
            start_date: value.start_date,
            end_date: value.end_date,
            created_at: value.created_at,
            updated_at: value.updated_at,
            tags: Vec::new(),
        }
    }
}

impl ExperienceResponse {
    pub fn from_experience_with_tags(
        exp: Experience,
        exp_tags: &Vec<(ExperienceTag, Tag)>,
    ) -> ExperienceResponse {
        ExperienceResponse {
            id: exp.id,
            title: exp.title,
            company_name: exp.company_name,
            description: exp.description,
            location: exp.location,
            start_date: exp.start_date,
            end_date: exp.end_date,
            created_at: exp.created_at,
            updated_at: exp.updated_at,
            tags: exp_tags
                .into_iter()
                .filter(|(et, _)| et.experience_id == exp.id)
                .map(|(_, t)| t.clone())
                .collect(),
        }
    }
}
