use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use crate::{
    PooledConn,
    modules::categories::{Category, NewCategory, UpdateCategory},
    schema::categories,
};

pub async fn one(conn: &mut PooledConn, slug: &str) -> Result<Option<Category>, Error> {
    categories::table
        .find(slug)
        .filter(categories::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .first::<Category>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<Category>, Error> {
    categories::table
        .filter(categories::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .order_by(categories::slug.desc())
        .load::<Category>(conn)
        .await
}

pub async fn insert(conn: &mut PooledConn, tag: NewCategory) -> Result<Category, Error> {
    diesel::insert_into(categories::table)
        .values(&tag)
        .returning(Category::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    slug: &str,
    category: UpdateCategory,
) -> Result<Option<Category>, Error> {
    diesel::update(categories::dsl::categories.find(slug))
        .set(category)
        .returning(Category::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, slug: &str) -> Result<Option<Category>, Error> {
    diesel::update(categories::dsl::categories.find(slug))
        .set(categories::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(Category::as_returning())
        .get_result(conn)
        .await
        .optional()
}
