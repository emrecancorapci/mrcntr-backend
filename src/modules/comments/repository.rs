use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use super::{Comment, NewComment, UpdateComment};
use crate::{PooledConn, schema::comments};

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<Comment>, Error> {
    comments::table
        .filter(comments::id.eq(id))
        .first::<Comment>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<Comment>, Error> {
    comments::table
        .order_by(comments::id.desc())
        .load::<Comment>(conn)
        .await
}

pub async fn insert(conn: &mut PooledConn, comment: NewComment) -> Result<Comment, Error> {
    diesel::insert_into(comments::table)
        .values(&comment)
        .returning(Comment::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    id: i32,
    comment: UpdateComment,
) -> Result<Option<Comment>, Error> {
    diesel::update(comments::dsl::comments.find(id))
        .set(comment)
        .returning(Comment::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<Comment>, Error> {
    diesel::delete(comments::dsl::comments.filter(comments::id.eq(id)))
        .returning(Comment::as_returning())
        .get_result(conn)
        .await
        .optional()
}
