use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, result::Error,
};

use super::{NewProjectAiUsage, ProjectAiUsage, UpdateProjectAiUsage};
use crate::{PooledConn, schema::project_ai_usages};

pub fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectAiUsage>, Error> {
    project_ai_usages::table
        .filter(project_ai_usages::id.eq(id))
        .first::<ProjectAiUsage>(conn)
        .optional()
}

pub fn many(conn: &mut PooledConn) -> Result<Vec<ProjectAiUsage>, Error> {
    project_ai_usages::table
        .order_by(project_ai_usages::id.desc())
        .load::<ProjectAiUsage>(conn)
}

pub fn insert(conn: &mut PooledConn, project: NewProjectAiUsage) -> Result<ProjectAiUsage, Error> {
    diesel::insert_into(project_ai_usages::table)
        .values(&project)
        .returning(ProjectAiUsage::as_returning())
        .get_result(conn)
}

pub fn update(
    conn: &mut PooledConn,
    id: i32,
    project: UpdateProjectAiUsage,
) -> Result<Option<ProjectAiUsage>, Error> {
    diesel::update(project_ai_usages::dsl::project_ai_usages.find(id))
        .set(project)
        .returning(ProjectAiUsage::as_returning())
        .get_result(conn)
        .optional()
}

pub fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectAiUsage>, Error> {
    diesel::delete(project_ai_usages::dsl::project_ai_usages.filter(project_ai_usages::id.eq(id)))
        .returning(ProjectAiUsage::as_returning())
        .get_result(conn)
        .optional()
}
