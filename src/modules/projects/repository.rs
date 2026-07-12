use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use super::{
    NewProject, Project, UpdateProject,
    modules::{
        project_ai_usages::ProjectAiUsage, project_statuses::ProjectStatus,
        project_types::ProjectType,
    },
};
use crate::{
    PooledConn,
    schema::{self, projects},
};

pub async fn one(
    conn: &mut PooledConn,
    project_id: i32,
) -> Result<Option<(Project, Option<ProjectStatus>, Option<ProjectType>, Option<ProjectAiUsage>)>, Error> {
    projects::table
        .find(project_id)
        .filter(projects::deleted_at.is_null())
        .left_join(schema::project_statuses::table)
        .left_join(schema::project_types::table)
        .left_join(schema::project_ai_usages::table)
        .first(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<Project>, Error> {
    projects::table
        .filter(projects::deleted_at.is_null())
        .order_by(projects::id.desc())
        .load::<Project>(conn)
        .await
}

pub async fn insert(conn: &mut PooledConn, project: NewProject) -> Result<Project, Error> {
    diesel::insert_into(projects::table)
        .values(project)
        .returning(Project::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    id: i32,
    project: UpdateProject,
) -> Result<Option<Project>, Error> {
    diesel::update(projects::dsl::projects.find(id))
        .set(project)
        .returning(Project::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<Project>, Error> {
    diesel::update(projects::dsl::projects.find(id))
        .set(projects::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(Project::as_returning())
        .get_result(conn)
        .await
        .optional()
}
