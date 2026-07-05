use super::{NewProjectLink, ProjectLink, UpdateProjectLink};
use crate::{PooledConn, schema::project_links};

use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectLink>, Error> {
    project_links::table
        .find(id)
        .filter(project_links::deleted_at.eq(Option::<DateTime<Utc>>::None))
        .first::<ProjectLink>(conn)
        .await
        .optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<ProjectLink>, Error> {
    project_links::table
        .order_by(project_links::id.desc())
        .load::<ProjectLink>(conn)
        .await
}

pub async fn insert(
    conn: &mut PooledConn,
    project_link: Vec<NewProjectLink>,
) -> Result<Vec<ProjectLink>, Error> {
    diesel::insert_into(project_links::table)
        .values(project_link)
        .returning(ProjectLink::as_returning())
        .get_results(conn)
        .await
}

pub async fn update(
    conn: &mut PooledConn,
    id: i32,
    project_link: UpdateProjectLink,
) -> Result<Option<ProjectLink>, Error> {
    diesel::update(project_links::dsl::project_links.find(id))
        .set(project_link)
        .returning(ProjectLink::as_returning())
        .get_result(conn)
        .await
        .optional()
}

pub async fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<ProjectLink>, Error> {
    diesel::update(project_links::dsl::project_links.find(id))
        .set(project_links::deleted_at.eq(Option::<DateTime<Utc>>::Some(Utc::now())))
        .returning(ProjectLink::as_returning())
        .get_result(conn)
        .await
        .optional()
}
