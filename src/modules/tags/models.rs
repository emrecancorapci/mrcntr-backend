use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize,
};
use serde::{Deserialize, Serialize};
use std::io::Write;
use validator::Validate;

#[derive(Queryable, Selectable, Identifiable, Validate, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: i32,
    #[validate(length(min = 1))]
    pub slug: String,
    #[validate(length(min = 1))]
    pub title: String,
    pub tag_type: Option<TagTypeEnum>,
    #[validate(range(min = 1))]
    pub proficiency: Option<i16>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTag {
    pub slug: String,
    pub title: String,
    pub tag_type: Option<TagTypeEnum>,
    pub proficiency: Option<i16>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(AsChangeset, Debug, Clone, Deserialize)]
#[diesel(table_name = schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateTag {
    pub slug: String,
    pub title: String,
    pub tag_type: Option<TagTypeEnum>,
    pub proficiency: Option<i16>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(AsExpression, FromSqlRow, Debug, Serialize, Deserialize, Clone)]
#[diesel(sql_type = schema::sql_types::TagTypes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub enum TagTypeEnum {
    Language,
    Tech,
    Tool,
    Os,
    Feature,
}

impl serialize::ToSql<schema::sql_types::TagTypes, diesel::pg::Pg> for TagTypeEnum {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        match self {
            TagTypeEnum::Language => out.write_all(b"language"),
            TagTypeEnum::Tech => out.write_all(b"tech"),
            TagTypeEnum::Tool => out.write_all(b"tool"),
            TagTypeEnum::Os => out.write_all(b"os"),
            TagTypeEnum::Feature => out.write_all(b"feature"),
        }?;

        Ok(serialize::IsNull::No)
    }
}

impl FromSql<schema::sql_types::TagTypes, diesel::pg::Pg> for TagTypeEnum {
    fn from_sql(
        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"language" => Ok(TagTypeEnum::Language),
            b"tech" => Ok(TagTypeEnum::Tech),
            b"tool" => Ok(TagTypeEnum::Tool),
            b"os" => Ok(TagTypeEnum::Os),
            b"feature" => Ok(TagTypeEnum::Feature),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
