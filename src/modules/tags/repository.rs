use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use super::{NewTag, Tag, UpdateTag};
use crate::{PooledConn, schema::tags};

pub async fn one(conn: &mut PooledConn, id: &i32) -> Result<Option<Tag>, Error> {
    tags::table.find(id).first::<Tag>(conn).await.optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<Tag>, Error> {
    tags::table
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
    diesel::delete(tags::dsl::tags.filter(tags::id.eq(id)))
        .returning(Tag::as_returning())
        .get_result(conn)
        .await
        .optional()
}
