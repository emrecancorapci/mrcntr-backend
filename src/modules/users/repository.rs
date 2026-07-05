use super::{NewUser, UpdateUser, User};
use crate::{PooledConn, schema::users};

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn one(conn: &mut PooledConn, uuid: Uuid) -> Result<Option<User>, Error> {
    users::table
        .find(uuid)
        .filter(users::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .first::<User>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<User>, Error> {
    users::table
        .filter(users::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .order_by(users::created_at.desc())
        .load::<User>(conn)
        .await
        .map(|v| v.into_iter().map(|u| u.into()).collect())
}

pub async fn insert(conn: &mut PooledConn, experience: NewUser) -> Result<User, Error> {
    diesel::insert_into(users::table)
        .values(&experience)
        .returning(User::as_returning())
        .get_result(conn)
        .await
        .map(|u| u.into())
}

pub async fn update(
    conn: &mut PooledConn,
    uuid: Uuid,
    user: UpdateUser,
) -> Result<Option<User>, Error> {
    diesel::update(users::dsl::users.find(uuid))
        .set(user)
        .returning(User::as_returning())
        .get_result(conn)
        .await
        .map(|u| u.into())
        .optional()
}

pub async fn delete(conn: &mut PooledConn, uuid: Uuid) -> Result<Option<User>, Error> {
    diesel::update(users::dsl::users.find(uuid))
        .set(users::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .returning(User::as_returning())
        .get_result(conn)
        .await
        .optional()
}
