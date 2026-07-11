use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema;

#[derive(
    Identifiable, Insertable, Selectable, Queryable, Associations, Serialize, Deserialize, Debug, ToSchema,
)]
#[diesel(table_name = schema::experiences_tags)]
#[diesel(belongs_to(crate::modules::experiences::Experience))]
#[diesel(belongs_to(crate::modules::tags::Tag))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(tag_id, experience_id))]
pub struct ExperienceTag {
    pub experience_id: i32,
    pub tag_id: i32,
    pub sort_order: Option<i16>,
}

#[derive(Deserialize, ToSchema)]
pub struct InsertManyExperienceTagsItem {
    pub tag_id: i32,
    pub sort: Option<i16>,
}

#[derive(Deserialize, ToSchema)]
pub struct InsertManyExperienceTagsBody {
    pub experience_id: i32,
    pub tags: Vec<InsertManyExperienceTagsItem>,
}
