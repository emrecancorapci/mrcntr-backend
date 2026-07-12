use super::{NewProjectType, ProjectType, UpdateProjectType};
use crate::{PooledConn, schema::project_types};

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectType>, Error> {
    project_types::table
        .find(id)
        .filter(project_types::deleted_at.is_null())
        .first::<ProjectType>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<ProjectType>, Error> {
    project_types::table
        .filter(project_types::deleted_at.is_null())
        .order_by(project_types::id.desc())
        .load::<ProjectType>(conn)
        .await
}

pub async fn insert(
    conn: &mut PooledConn,
    project_type: NewProjectType,
) -> Result<ProjectType, Error> {
    diesel::insert_into(project_types::table)
        .values(&project_type)
        .returning(ProjectType::as_returning())
        .get_result(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    id: i32,
    project_type: UpdateProjectType,
) -> Result<Option<ProjectType>, Error> {
    diesel::update(project_types::dsl::project_types.find(id))
        .set(project_type)
        .returning(ProjectType::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectType>, Error> {
    diesel::update(project_types::dsl::project_types.find(id))
        .set(project_types::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(ProjectType::as_returning())
        .get_result(conn)
        .await
        .optional()
}
