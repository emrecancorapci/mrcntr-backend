use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use super::{NewProjectBlock, ProjectBlock, UpdateProjectBlock};
use crate::{PooledConn, schema::project_blocks};

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectBlock>, Error> {
    project_blocks::table
        .filter(project_blocks::id.eq(id))
        .first::<ProjectBlock>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<ProjectBlock>, Error> {
    project_blocks::table
        .order_by(project_blocks::id.desc())
        .load::<ProjectBlock>(conn)
        .await
}

pub async fn insert(
    conn: &mut PooledConn,
    project_block: Vec<NewProjectBlock>,
) -> Result<Vec<ProjectBlock>, Error> {
    diesel::insert_into(project_blocks::table)
        .values(project_block)
        .returning(ProjectBlock::as_returning())
        .get_results(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    id: i32,
    project_block: UpdateProjectBlock,
) -> Result<Option<ProjectBlock>, Error> {
    diesel::update(project_blocks::dsl::project_blocks.find(id))
        .set(project_block)
        .returning(ProjectBlock::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectBlock>, Error> {
    diesel::delete(project_blocks::dsl::project_blocks.filter(project_blocks::id.eq(id)))
        .returning(ProjectBlock::as_returning())
        .get_result(conn)
        .await
        .optional()
}
