use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};

use super::{NewProject, Project, UpdateProject};
use crate::{PooledConn, schema::projects};

pub fn one(conn: &mut PooledConn, id: i32) -> Result<Option<Project>, Error> {
    projects::table
        .filter(projects::id.eq(id))
        .first::<Project>(conn)
        .optional()
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
