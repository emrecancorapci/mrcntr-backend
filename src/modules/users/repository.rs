use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};
use uuid::Uuid;

use super::{User, NewUser};
use crate::{PooledConn, modules::users::UpdateUser, schema::users};

pub fn one(conn: &mut PooledConn, uuid: Uuid) -> Result<Option<User>, Error> {
    users::table
        .find(uuid)
        .first::<User>(conn)
        .optional()
}

pub fn one_by_email(conn: &mut PooledConn, email: &str) -> Result<Option<User>, Error> {
    users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<User>, Error> {
    users::table
        .order_by(users::created_at.desc())
        .load::<User>(conn)
}

pub fn insert(conn: &mut PooledConn, experience: NewUser) -> Result<User, Error> {
    diesel::insert_into(users::table)
        .values(&experience)
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn update(
    conn: &mut PooledConn,
    uuid: Uuid,
    user: UpdateUser,
) -> Result<Option<User>, Error> {
    diesel::update(users::dsl::users.find(uuid))
        .set(user)
        .returning(User::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, uuid: Uuid) -> Result<Option<User>, Error> {
    diesel::delete(users::dsl::users.find(uuid))
        .returning(User::as_returning())
        .get_result(conn)
        .optional()
}
