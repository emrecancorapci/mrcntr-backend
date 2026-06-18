use diesel::{
    BelongingToDsl, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    result::Error,
};

use super::{
    MergedProject, NewProject, Project, UpdateProject,
    modules::{
        project_ai_usages::ProjectAiUsage, project_blocks::ProjectBlock,
        project_links::ProjectLink, project_statuses::ProjectStatus, project_types::ProjectType,
    },
};
use crate::{
    PooledConn,
    schema::{self, projects},
};

pub fn one(conn: &mut PooledConn, project_id: i32) -> Result<Option<MergedProject>, Error> {
    let project_data: Option<(Project, ProjectStatus, ProjectType, ProjectAiUsage)> =
        projects::table
            .find(project_id)
            .inner_join(schema::project_statuses::table)
            .inner_join(schema::project_types::table)
            .inner_join(schema::project_ai_usages::table)
            .first(conn)
            .optional()?;

    let (project, status, p_type, ai_usage) = match project_data {
        Some(data) => data,
        None => return Ok(None),
    };

    let blocks = ProjectBlock::belonging_to(&project).load::<ProjectBlock>(conn)?;
    let links = ProjectLink::belonging_to(&project).load::<ProjectLink>(conn)?;

    Ok(Some(MergedProject {
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
        project_type: p_type,
        project_ai_usage: ai_usage,
        project_blocks: blocks,
        project_links: links,
        project_status: status,
    }))
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<Project>, Error> {
    projects::table
        .order_by(projects::id.desc())
        .load::<Project>(conn)
}

pub fn insert(conn: &mut PooledConn, project: NewProject) -> Result<Project, Error> {
    diesel::insert_into(projects::table)
        .values(&project)
        .returning(Project::as_returning())
        .get_result(conn)
}

pub fn update(
    conn: &mut PooledConn,
    id: i32,
    project: UpdateProject,
) -> Result<Option<Project>, Error> {
    diesel::update(projects::dsl::projects.find(id))
        .set(project)
        .returning(Project::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<Project>, Error> {
    diesel::delete(projects::dsl::projects.filter(projects::id.eq(id)))
        .returning(Project::as_returning())
        .get_result(conn)
        .optional()
}
