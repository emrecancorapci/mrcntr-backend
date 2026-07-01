use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use super::{NewProjectStatus, ProjectStatus, UpdateProjectStatus};
use crate::{PooledConn, schema::project_statuses};

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectStatus>, Error> {
    project_statuses::table
        .filter(project_statuses::id.eq(id))
        .first::<ProjectStatus>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<ProjectStatus>, Error> {
    project_statuses::table
        .order_by(project_statuses::id.desc())
        .load::<ProjectStatus>(conn)
        .await
}

pub async fn insert(
    conn: &mut PooledConn,
    project_status: NewProjectStatus,
) -> Result<ProjectStatus, Error> {
    diesel::insert_into(project_statuses::table)
        .values(&project_status)
        .returning(ProjectStatus::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    id: i32,
    project_status: UpdateProjectStatus,
) -> Result<Option<ProjectStatus>, Error> {
    diesel::update(project_statuses::dsl::project_statuses.find(id))
        .set(project_status)
        .returning(ProjectStatus::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectStatus>, Error> {
    diesel::delete(project_statuses::dsl::project_statuses.filter(project_statuses::id.eq(id)))
        .returning(ProjectStatus::as_returning())
        .get_result(conn)
        .await
        .optional()
}
