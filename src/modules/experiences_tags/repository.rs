use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};

use super::ExperienceTag;
use crate::{PooledConn, schema::experiences_tags};

pub fn many_by_experience_id(
    conn: &mut PooledConn,
    experience_id: i32,
) -> Result<Vec<ExperienceTag>, Error> {
    experiences_tags::table
        .filter(experiences_tags::experience_id.eq(experience_id))
        .get_results(conn)
}

pub fn many_by_tag_id(conn: &mut PooledConn, tag_id: i32) -> Result<Vec<ExperienceTag>, Error> {
    experiences_tags::table
        .filter(experiences_tags::tag_id.eq(tag_id))
        .get_results(conn)
}

pub fn one(
    conn: &mut PooledConn,
    tag_id: i32,
    experience_id: i32,
) -> Result<Option<ExperienceTag>, Error> {
    experiences_tags::table
        .filter(experiences_tags::tag_id.eq(tag_id))
        .filter(experiences_tags::experience_id.eq(experience_id))
        .get_result(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<ExperienceTag>, Error> {
    experiences_tags::table.get_results(conn)
}

pub fn insert(conn: &mut PooledConn, experience: ExperienceTag) -> Result<ExperienceTag, Error> {
    diesel::insert_into(experiences_tags::table)
        .values(&experience)
        .returning(ExperienceTag::as_returning())
        .get_result(conn)
}

pub fn delete_by_experience_id(
    conn: &mut PooledConn,
    experience_id: i32,
) -> Result<Vec<ExperienceTag>, Error> {
    diesel::delete(
        experiences_tags::dsl::experiences_tags
            .filter(experiences_tags::experience_id.eq(experience_id)),
    )
    .returning(ExperienceTag::as_returning())
    .get_results(conn)
}

pub fn delete_by_tag_id(conn: &mut PooledConn, tag_id: i32) -> Result<Vec<ExperienceTag>, Error> {
    diesel::delete(
        experiences_tags::dsl::experiences_tags.filter(experiences_tags::tag_id.eq(tag_id)),
    )
    .returning(ExperienceTag::as_returning())
    .get_results(conn)
}
