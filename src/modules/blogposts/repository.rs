use diesel::{
    BelongingToDsl, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
	result::Error,
};

use super::{NewBlogpost, Blogpost, UpdateBlogpost};
use crate::{PooledConn, schema::blogposts};

pub fn one(conn: &mut PooledConn, id: i32) -> Result<Option<Blogpost>, Error> {
    blogposts::table
        .filter(blogposts::id.eq(id))
        .first::<Blogpost>(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<Blogpost>, Error> {
    blogposts::table
        .order_by(blogposts::id.desc())
        .load::<Blogpost>(conn)
}

pub fn insert(conn: &mut PooledConn, blogpost: NewBlogpost) -> Result<Blogpost, Error> {
    diesel::insert_into(blogposts::table)
        .values(&blogpost)
        .returning(Blogpost::as_returning())
        .get_result(conn)
}

pub fn update(
    conn: &mut PooledConn,
    id: i32,
    blogpost: UpdateBlogpost,
) -> Result<Option<Blogpost>, Error> {
    diesel::update(blogposts::dsl::blogposts.find(id))
        .set(blogpost)
        .returning(Blogpost::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<Blogpost>, Error> {
    diesel::delete(blogposts::dsl::blogposts.filter(blogposts::id.eq(id)))
        .returning(Blogpost::as_returning())
        .get_result(conn)
        .optional()
}
