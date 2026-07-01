use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use uuid::Uuid;

use super::{NewUser, User};
use crate::{
    PooledConn,
    modules::{
        roles::Role,
        users::{UpdateUser, UserResponse},
    },
    schema::{self, roles, users},
};

pub async fn one(conn: &mut PooledConn, uuid: Uuid) -> Result<Option<UserResponse>, Error> {
    users::table
        .find(uuid)
        .inner_join(schema::roles::table)
        .select((users::all_columns, roles::all_columns))
        .first::<(User, Role)>(conn)
        .await
        .map(|(u, r)| {
            let mut u: UserResponse = u.into();
            u.role = Some(r.title);

            u
        })
        .optional()
}

pub async fn one_by_email(conn: &mut PooledConn, email: &str) -> Result<Option<User>, Error> {
    users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<UserResponse>, Error> {
    users::table
        .order_by(users::created_at.desc())
        .load::<User>(conn)
        .await
        .map(|v| v.into_iter().map(|u| u.into()).collect())
}

pub async fn insert(conn: &mut PooledConn, experience: NewUser) -> Result<UserResponse, Error> {
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
) -> Result<Option<UserResponse>, Error> {
    diesel::update(users::dsl::users.find(uuid))
        .set(user)
        .returning(User::as_returning())
        .get_result(conn)
        .await
        .map(|u| u.into())
        .optional()
}

pub async fn delete(conn: &mut PooledConn, uuid: Uuid) -> Result<Option<UserResponse>, Error> {
    diesel::delete(users::dsl::users.find(uuid))
        .returning(User::as_returning())
        .get_result(conn)
        .await
        .map(|u| u.into())
        .optional()
}
