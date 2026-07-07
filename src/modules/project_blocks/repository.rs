use super::{NewProjectBlock, ProjectBlock, UpdateProjectBlock};
use crate::{PooledConn, modules::projects::Project, schema::project_blocks};

use chrono::{DateTime, Utc};
use diesel::{
    BelongingToDsl, BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl,
    SelectableHelper, result::Error,
};
use diesel_async::RunQueryDsl;

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectBlock>, Error> {
    project_blocks::table
        .find(id)
        .filter(project_blocks::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .first::<ProjectBlock>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<ProjectBlock>, Error> {
    project_blocks::table
        .filter(project_blocks::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .order_by(project_blocks::id.desc())
        .load::<ProjectBlock>(conn)
        .await
}

pub async fn many_by_project(
    conn: &mut PooledConn,
    project: &Project,
) -> Result<Vec<ProjectBlock>, Error> {
    ProjectBlock::belonging_to(project)
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
    diesel::update(project_blocks::dsl::project_blocks.find(id))
        .set(project_blocks::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(ProjectBlock::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete_by_project_id(
    conn: &mut PooledConn,
    project_id: i32,
) -> Result<Vec<ProjectBlock>, Error> {
    diesel::update(
        project_blocks::dsl::project_blocks.filter(project_blocks::project_id.eq(project_id)),
    )
    .set(project_blocks::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
    .returning(ProjectBlock::as_returning())
    .get_results(conn)
    .await
}

pub async fn many_by_project_id(
    conn: &mut PooledConn,
    project_id: &i32,
) -> Result<Vec<ProjectBlock>, Error> {
    project_blocks::table
        .filter(
            project_blocks::deleted_at
                .eq(Option::<DateTime<Utc>>::None)
                .and(project_blocks::project_id.eq(project_id)),
        )
        .order_by(project_blocks::id.desc())
        .load::<ProjectBlock>(conn)
        .await
}