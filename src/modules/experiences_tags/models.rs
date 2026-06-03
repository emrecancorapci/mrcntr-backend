use diesel::prelude::*;

use crate::modules::{experiences::*, tags::*};
use crate::schema;

#[derive(Identifiable, Insertable, Selectable, Queryable, Associations, Debug)]
#[diesel(table_name = schema::experiences_tags)]
#[diesel(belongs_to(Experience))]
#[diesel(belongs_to(Tag))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(tag_id, experience_id))]
pub struct ExperienceTag {
    pub experience_id: i32,
    pub tag_id: i32,
    pub sort_order: Option<i16>,
}
