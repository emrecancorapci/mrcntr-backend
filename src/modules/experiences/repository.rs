use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use super::{Experience, NewExperience, UpdateExperience};
use crate::{PooledConn, schema::experiences};

pub async fn one(conn: &mut PooledConn, id: &i32) -> Result<Option<Experience>, Error> {
    experiences::table
        .find(id)
        .filter(experiences::deleted_at.is_null())
        .first::<Experience>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<Experience>, Error> {
    experiences::table
        .filter(experiences::deleted_at.is_null())
        .order_by(experiences::start_date.desc())
        .load::<Experience>(conn)
        .await
}

pub async fn insert(conn: &mut PooledConn, experience: NewExperience) -> Result<Experience, Error> {
    diesel::insert_into(experiences::table)
        .values(&experience)
        .returning(Experience::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    id: &i32,
    experience: UpdateExperience,
) -> Result<Option<Experience>, Error> {
    diesel::update(experiences::dsl::experiences.find(id))
        .set(experience)
        .returning(Experience::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: &i32) -> Result<Option<Experience>, Error> {
    diesel::update(experiences::dsl::experiences.find(id))
        .set(experiences::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(Experience::as_returning())
        .get_result(conn)
        .await
        .optional()
}
