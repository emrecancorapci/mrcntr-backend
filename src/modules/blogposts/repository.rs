use super::{Blogpost, NewBlogpost, UpdateBlogpost};
use crate::{PooledConn, schema::blogposts};

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<Blogpost>, Error> {
    blogposts::table
        .find(id)
        .filter(blogposts::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .first::<Blogpost>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<Blogpost>, Error> {
    blogposts::table
        .filter(blogposts::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .order_by(blogposts::id.desc())
        .load::<Blogpost>(conn)
        .await
}

pub async fn insert(conn: &mut PooledConn, blogpost: NewBlogpost) -> Result<Blogpost, Error> {
    diesel::insert_into(blogposts::table)
        .values(&blogpost)
        .returning(Blogpost::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    id: i32,
    blogpost: UpdateBlogpost,
) -> Result<Option<Blogpost>, Error> {
    diesel::update(blogposts::dsl::blogposts.find(id))
        .set(blogpost)
        .returning(Blogpost::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<Blogpost>, Error> {
    diesel::update(blogposts::dsl::blogposts.find(id))
        .set(blogposts::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(Blogpost::as_returning())
        .get_result(conn)
        .await
        .optional()
}
