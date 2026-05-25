use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};

use super::{Experience, NewExperience};
use crate::{PooledConn, modules::experiences::models::UpdateExperience, schema::experiences};

pub fn one(conn: &mut PooledConn, id: &i32) -> Result<Option<Experience>, Error> {
    experiences::table
        .find(id)
        .first::<Experience>(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<Experience>, Error> {
    experiences::table
        .order_by(experiences::start_date.desc())
        .load::<Experience>(conn)
}

pub fn insert(conn: &mut PooledConn, experience: NewExperience) -> Result<Experience, Error> {
    diesel::insert_into(experiences::table)
        .values(&experience)
        .returning(Experience::as_returning())
        .get_result(conn)
}

pub fn update(
    conn: &mut PooledConn,
    id: &i32,
    experience: UpdateExperience,
) -> Result<Option<Experience>, Error> {
    diesel::update(experiences::dsl::experiences.find(id))
        .set(experience)
        .returning(Experience::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, id: &i32) -> Result<Option<Experience>, Error> {
    diesel::delete(experiences::dsl::experiences.filter(experiences::id.eq(id)))
        .returning(Experience::as_returning())
        .get_result(conn)
        .optional()
}
