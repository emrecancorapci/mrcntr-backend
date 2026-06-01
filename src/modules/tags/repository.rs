use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};

use super::{Tag, UpdateTag};
use crate::{PooledConn, schema::tags};

pub fn one(conn: &mut PooledConn, id: &i32) -> Result<Option<Tag>, Error> {
    tags::table.find(id).first::<Tag>(conn).optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<Tag>, Error> {
    tags::table.order_by(tags::id.desc()).load::<Tag>(conn)
}

pub fn insert(conn: &mut PooledConn, tag: Tag) -> Result<Tag, Error> {
    diesel::insert_into(tags::table)
        .values(&tag)
        .returning(Tag::as_returning())
        .get_result(conn)
}

pub fn update(conn: &mut PooledConn, id: &i32, tag: UpdateTag) -> Result<Option<Tag>, Error> {
    diesel::update(tags::dsl::tags.find(id))
        .set(tag)
        .returning(Tag::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, id: &i32) -> Result<Option<Tag>, Error> {
    diesel::delete(tags::dsl::tags.filter(tags::id.eq(id)))
        .returning(Tag::as_returning())
        .get_result(conn)
        .optional()
}
