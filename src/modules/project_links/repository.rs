use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};

use super::{NewProjectLink, ProjectLink, UpdateProjectLink};
use crate::{PooledConn, schema::project_links};

pub fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectLink>, Error> {
    project_links::table
        .filter(project_links::id.eq(id))
        .first::<ProjectLink>(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<ProjectLink>, Error> {
    project_links::table
        .order_by(project_links::id.desc())
        .load::<ProjectLink>(conn)
}

pub fn insert(conn: &mut PooledConn, project: NewProjectLink) -> Result<ProjectLink, Error> {
    diesel::insert_into(project_links::table)
        .values(&project)
        .returning(ProjectLink::as_returning())
        .get_result(conn)
}

pub fn update(
    conn: &mut PooledConn,
    id: i32,
    project: UpdateProjectLink,
) -> Result<Option<ProjectLink>, Error> {
    diesel::update(project_links::dsl::project_links.find(id))
        .set(project)
        .returning(ProjectLink::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectLink>, Error> {
    diesel::delete(project_links::dsl::project_links.filter(project_links::id.eq(id)))
        .returning(ProjectLink::as_returning())
        .get_result(conn)
        .optional()
}
