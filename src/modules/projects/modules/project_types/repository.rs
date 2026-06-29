use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};

use super::{NewProjectType, ProjectType, UpdateProjectType};
use crate::{PooledConn, schema::project_types};

pub fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectType>, Error> {
    project_types::table
        .filter(project_types::id.eq(id))
        .first::<ProjectType>(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<ProjectType>, Error> {
    project_types::table
        .order_by(project_types::id.desc())
        .load::<ProjectType>(conn)
}

pub fn insert(conn: &mut PooledConn, project_type: NewProjectType) -> Result<ProjectType, Error> {
    diesel::insert_into(project_types::table)
        .values(&project_type)
        .returning(ProjectType::as_returning())
        .get_result(conn)
}

pub fn update(
    conn: &mut PooledConn,
    id: i32,
    project_type: UpdateProjectType,
) -> Result<Option<ProjectType>, Error> {
    diesel::update(project_types::dsl::project_types.find(id))
        .set(project_type)
        .returning(ProjectType::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectType>, Error> {
    diesel::delete(project_types::dsl::project_types.filter(project_types::id.eq(id)))
        .returning(ProjectType::as_returning())
        .get_result(conn)
        .optional()
}
