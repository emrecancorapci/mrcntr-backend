use crate::{
    PooledConn,
    modules::users::{NewUser, User},
    schema::users,
};

use chrono::{DateTime, Utc};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, SelectableHelper,
    query_dsl::methods::FilterDsl, result::Error,
};
use diesel_async::RunQueryDsl;

pub async fn one_by_email(conn: &mut PooledConn, email: &str) -> Result<Option<User>, Error> {
    users::table
        .filter(
            users::email
                .eq(email)
                .and(users::deleted_at.is_null()),
        )
        .first::<User>(conn)
        .await
        .optional()
}

pub async fn insert(conn: &mut PooledConn, experience: NewUser) -> Result<User, Error> {
    diesel::insert_into(users::table)
        .values(&experience)
        .returning(User::as_returning())
        .get_result(conn)
        .await
        .map(|u| u.into())
}
