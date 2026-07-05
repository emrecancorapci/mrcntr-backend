use chrono::{DateTime, Utc};
use diesel::{
    BelongingToDsl, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error,
};
use diesel_async::RunQueryDsl;

use super::{
    NewProject, Project, ProjectResponse, UpdateProject,
    modules::{
        project_ai_usages::ProjectAiUsage, project_blocks::ProjectBlock,
        project_links::ProjectLink, project_statuses::ProjectStatus, project_types::ProjectType,
    },
};
use crate::{
    PooledConn,
    schema::{self, projects},
};

pub async fn one(conn: &mut PooledConn, project_id: i32) -> Result<Option<ProjectResponse>, Error> {
    let project_data: Option<(Project, ProjectStatus, ProjectType, ProjectAiUsage)> =
        projects::table
            .find(project_id)
            .filter(projects::deleted_at.eq(Option::<DateTime<Utc>>::None))
            .inner_join(schema::project_statuses::table)
            .inner_join(schema::project_types::table)
            .inner_join(schema::project_ai_usages::table)
            .first(conn)
            .await
            .optional()?;

    let (project, status, p_type, ai_usage) = match project_data {
        Some(data) => data,
        None => return Ok(None),
    };

    let blocks = ProjectBlock::belonging_to(&project)
        .load::<ProjectBlock>(conn)
        .await?;
    let links = ProjectLink::belonging_to(&project)
        .load::<ProjectLink>(conn)
        .await?;

    Ok(Some(ProjectResponse {
        id: project.id,
        title: project.title,
        project_description: project.project_description,
        content: project.content,
        year_created_at: project.year_created_at,
        latest_version: project.latest_version,
        is_visible: project.is_visible,
        is_featured: project.is_featured,
        project_status_id: project.project_status_id,
        project_type_id: project.project_type_id,
        project_ai_usage_id: project.project_ai_usage_id,
        created_at: project.created_at,
        updated_at: project.updated_at,
        project_type: Some(p_type),
        project_ai_usage: Some(ai_usage),
        project_blocks: blocks,
        project_links: links,
        project_status: Some(status),
        published_at: project.published_at,
        deleted_at: project.deleted_at,
    }))
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<Project>, Error> {
    projects::table
        .filter(projects::deleted_at.eq(Option::<DateTime<Utc>>::None))
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
