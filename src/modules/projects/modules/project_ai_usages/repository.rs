use super::{NewProjectAiUsage, ProjectAiUsage, UpdateProjectAiUsage};
use crate::{PooledConn, schema::project_ai_usages};

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectAiUsage>, Error> {
    project_ai_usages::table
        .find(id)
        .filter(project_ai_usages::deleted_at.is_null())
        .first::<ProjectAiUsage>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<ProjectAiUsage>, Error> {
    project_ai_usages::table
        .filter(project_ai_usages::deleted_at.is_null())
        .order_by(project_ai_usages::id.desc())
        .load::<ProjectAiUsage>(conn)
        .await
}

pub async fn insert(
    conn: &mut PooledConn,
    project_ai_usage: NewProjectAiUsage,
) -> Result<ProjectAiUsage, Error> {
    diesel::insert_into(project_ai_usages::table)
        .values(&project_ai_usage)
        .returning(ProjectAiUsage::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    id: i32,
    project_ai_usage: UpdateProjectAiUsage,
) -> Result<Option<ProjectAiUsage>, Error> {
    diesel::update(project_ai_usages::dsl::project_ai_usages.find(id))
        .set(project_ai_usage)
        .returning(ProjectAiUsage::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectAiUsage>, Error> {
    diesel::update(project_ai_usages::dsl::project_ai_usages.find(id))
        .set(project_ai_usages::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(ProjectAiUsage::as_returning())
        .get_result(conn)
        .await
        .optional()
}
