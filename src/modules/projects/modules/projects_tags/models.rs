use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::modules::{projects::*, tags::*};
use crate::schema;

#[derive(
    Identifiable, Insertable, Selectable, Queryable, Associations, Serialize, Deserialize, Debug,
)]
#[diesel(table_name = schema::projects_tags)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(Tag))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(tag_id, project_id))]
pub struct ProjectTag {
    pub project_id: i32,
    pub tag_id: i32,
    // pub sort_order: Option<i16>,
}

// #[derive(Deserialize)]
// pub struct InsertManyProjectTagsItem {
//     pub tag_id: i32,
//     pub sort: Option<i16>,
// }

#[derive(Deserialize)]
pub struct InsertManyProjectTagsBody {
    pub project_id: i32,
    pub tags: Vec<i32>, // Vec<InsertManyProjectTagsItem>
}
