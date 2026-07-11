use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::{projects::*, tags::*};
use crate::schema;

#[derive(
    Identifiable, Insertable, Selectable, Queryable, Associations, Serialize, Deserialize, Debug, ToSchema,
)]
#[diesel(table_name = schema::projects_tags)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(Tag))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(tag_id, project_id))]
pub struct ProjectTag {
    pub project_id: i32,
    pub tag_id: i32,
    pub is_featured: bool,
    pub sort_order: i16,
}

#[derive(Insertable, Serialize, Deserialize, Debug, ToSchema)]
#[diesel(table_name = schema::projects_tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProjectTag {
    pub project_id: i32,
    pub tag_id: i32,
    pub is_featured: Option<bool>,
    pub sort_order: i16,
}

#[derive(Deserialize)]
pub struct NewProjectTagRequest {
    pub tag_id: i32,
    pub is_featured: Option<bool>,
    pub sort_order: i16,
}

#[derive(Deserialize, ToSchema)]
pub struct TagInsertItem {
    pub tag_id: i32,
    pub is_featured: Option<bool>,
}

impl NewProjectTag {
    pub fn from_item(item: TagInsertItem, project_id: i32, index: i16) -> Self {
        Self {
            project_id,
            tag_id: item.tag_id,
            is_featured: item.is_featured,
            sort_order: index,
        }
    }

    pub fn from_request(req: NewProjectTagRequest, project_id: i32) -> Self {
        Self {
            project_id,
            tag_id: req.tag_id,
            is_featured: req.is_featured,
            sort_order: req.sort_order,
        }
    }
}
