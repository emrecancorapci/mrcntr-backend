use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};

use super::{NewProjectBlock, ProjectBlock, UpdateProjectBlock};
use crate::{PooledConn, schema::project_blocks};

pub fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectBlock>, Error> {
    project_blocks::table
        .filter(project_blocks::id.eq(id))
        .first::<ProjectBlock>(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<ProjectBlock>, Error> {
    project_blocks::table
        .order_by(project_blocks::id.desc())
        .load::<ProjectBlock>(conn)
}

pub fn insert(conn: &mut PooledConn, project: NewProjectBlock) -> Result<ProjectBlock, Error> {
    diesel::insert_into(project_blocks::table)
        .values(&project)
        .returning(ProjectBlock::as_returning())
        .get_result(conn)
}

pub fn update(
    conn: &mut PooledConn,
    id: i32,
    project: UpdateProjectBlock,
) -> Result<Option<ProjectBlock>, Error> {
    diesel::update(project_blocks::dsl::project_blocks.find(id))
        .set(project)
        .returning(ProjectBlock::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectBlock>, Error> {
    diesel::delete(project_blocks::dsl::project_blocks.filter(project_blocks::id.eq(id)))
        .returning(ProjectBlock::as_returning())
        .get_result(conn)
        .optional()
}
