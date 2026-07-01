use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use crate::{PooledConn, modules::categories::Category, schema::categories};

pub async fn one(conn: &mut PooledConn, slug: &str) -> Result<Option<Category>, Error> {
    categories::table
        .filter(categories::slug.eq(slug))
        .first::<Category>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<Category>, Error> {
    categories::table
        .order_by(categories::slug.desc())
        .load::<Category>(conn)
        .await
}

pub async fn insert(conn: &mut PooledConn, tag: Category) -> Result<Category, Error> {
    diesel::insert_into(categories::table)
        .values(&tag)
        .returning(Category::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    slug: &str,
    tag: &str,
) -> Result<Option<Category>, Error> {
    diesel::update(categories::dsl::categories.find(slug))
        .set(categories::title.eq(tag))
        .returning(Category::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, slug: &str) -> Result<Option<Category>, Error> {
    diesel::delete(categories::dsl::categories.filter(categories::slug.eq(slug)))
        .returning(Category::as_returning())
        .get_result(conn)
        .await
        .optional()
}
