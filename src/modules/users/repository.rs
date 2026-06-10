use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};
use uuid::Uuid;

use super::{NewUser, User};
use crate::{
    PooledConn,
    modules::users::{UpdateUser, UserResponse},
    schema::users,
};

pub fn one(conn: &mut PooledConn, uuid: Uuid) -> Result<Option<UserResponse>, Error> {
    users::table
        .find(uuid)
        .first::<User>(conn)
        .map(|u| u.to_response())
        .optional()
}

pub fn one_by_email(conn: &mut PooledConn, email: &str) -> Result<Option<User>, Error> {
    users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<UserResponse>, Error> {
    users::table
        .order_by(users::created_at.desc())
        .load::<User>(conn)
        .map(|v| v.into_iter().map(|u| u.to_response()).collect())
}

pub fn insert(conn: &mut PooledConn, experience: NewUser) -> Result<UserResponse, Error> {
    diesel::insert_into(users::table)
        .values(&experience)
        .returning(User::as_returning())
        .get_result(conn)
        .map(|u| u.to_response())
}

pub fn update(
    conn: &mut PooledConn,
    uuid: Uuid,
    user: UpdateUser,
) -> Result<Option<UserResponse>, Error> {
    diesel::update(users::dsl::users.find(uuid))
        .set(user)
        .returning(User::as_returning())
        .get_result(conn)
        .map(|u| u.to_response())
        .optional()
}

pub fn delete(conn: &mut PooledConn, uuid: Uuid) -> Result<Option<UserResponse>, Error> {
    diesel::delete(users::dsl::users.find(uuid))
        .returning(User::as_returning())
        .get_result(conn)
        .map(|u| u.to_response())
        .optional()
}
