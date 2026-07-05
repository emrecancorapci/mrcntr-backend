use super::{NewProjectStatus, ProjectStatus, UpdateProjectStatus};
use crate::{PooledConn, schema::project_statuses};

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectStatus>, Error> {
    project_statuses::table
        .find(id)
        .filter(project_statuses::deleted_at.eq(Option::<DateTime<Utc>>::None))
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
    diesel::update(project_statuses::dsl::project_statuses.find(id))
        .set(project_statuses::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(ProjectStatus::as_returning())
        .get_result(conn)
        .await
        .optional()
}
