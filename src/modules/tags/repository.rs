use super::{NewTag, Tag, UpdateTag};
use crate::{PooledConn, schema::tags};

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

pub async fn one(conn: &mut PooledConn, id: &i32) -> Result<Option<Tag>, Error> {
    tags::table
        .find(id)
        .filter(tags::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .first::<Tag>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<Tag>, Error> {
    tags::table
        .filter(tags::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .order_by(tags::id.desc())
        .load::<Tag>(conn)
        .await
}

pub async fn many_by_ids(conn: &mut PooledConn, ids: Vec<i32>) -> Result<Vec<Tag>, Error> {
    tags::table
        .filter(tags::id.eq_any(ids))
        .order_by(tags::id.desc())
        .load::<Tag>(conn)
        .await
}

pub async fn insert(conn: &mut PooledConn, tag: NewTag) -> Result<Tag, Error> {
    diesel::insert_into(tags::table)
        .values(&tag)
        .returning(Tag::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(conn: &mut PooledConn, id: &i32, tag: UpdateTag) -> Result<Option<Tag>, Error> {
    diesel::update(tags::dsl::tags.find(id))
        .set(tag)
        .returning(Tag::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: &i32) -> Result<Option<Tag>, Error> {
    diesel::update(tags::dsl::tags.find(id))
        .set(tags::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(Tag::as_returning())
        .get_result(conn)
        .await
        .optional()
}
