use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};

use super::{NewProjectStatus, ProjectStatus, UpdateProjectStatus};
use crate::{PooledConn, schema::project_statuses};

pub fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectStatus>, Error> {
    project_statuses::table
        .filter(project_statuses::id.eq(id))
        .first::<ProjectStatus>(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<ProjectStatus>, Error> {
    project_statuses::table
        .order_by(project_statuses::id.desc())
        .load::<ProjectStatus>(conn)
}

pub fn insert(conn: &mut PooledConn, project_status: NewProjectStatus) -> Result<ProjectStatus, Error> {
    diesel::insert_into(project_statuses::table)
        .values(&project_status)
        .returning(ProjectStatus::as_returning())
        .get_result(conn)
}

pub fn update(
    conn: &mut PooledConn,
    id: i32,
    project_status: UpdateProjectStatus,
) -> Result<Option<ProjectStatus>, Error> {
    diesel::update(project_statuses::dsl::project_statuses.find(id))
        .set(project_status)
        .returning(ProjectStatus::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectStatus>, Error> {
    diesel::delete(project_statuses::dsl::project_statuses.filter(project_statuses::id.eq(id)))
        .returning(ProjectStatus::as_returning())
        .get_result(conn)
        .optional()
}
