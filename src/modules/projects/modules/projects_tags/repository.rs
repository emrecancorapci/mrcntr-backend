use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
    result::Error,
};

use super::ProjectTag;
use crate::{PooledConn, schema::projects_tags};

// pub fn many_by_project_id(
//     conn: &mut PooledConn,
//     project_id: i32,
// ) -> Result<Vec<ProjectTag>, Error> {
//     projects_tags::table
//         .filter(projects_tags::project_id.eq(project_id))
//         .get_results(conn)
// }

// pub fn many_by_tag_id(conn: &mut PooledConn, tag_id: i32) -> Result<Vec<ProjectTag>, Error> {
//     projects_tags::table
//         .filter(projects_tags::tag_id.eq(tag_id))
//         .get_results(conn)
// }

// pub fn one(
//     conn: &mut PooledConn,
//     tag_id: i32,
//     project_id: i32,
// ) -> Result<Option<ProjectTag>, Error> {
//     projects_tags::table
//         .filter(projects_tags::tag_id.eq(tag_id))
//         .filter(projects_tags::project_id.eq(project_id))
//         .get_result(conn)
//         .optional()
// }

// pub fn many(conn: &mut PooledConn) -> Result<Vec<ProjectTag>, Error> {
//     projects_tags::table.get_results(conn)
// }

pub fn insert_one(conn: &mut PooledConn, project_tag: ProjectTag) -> Result<ProjectTag, Error> {
    diesel::insert_into(projects_tags::table)
        .values(&project_tag)
        .on_conflict((projects_tags::project_id, projects_tags::tag_id))
        .do_nothing()
        // .do_update()
        // .set(projects_tags::sort_order.eq(excluded(projects_tags::sort_order)))
        .returning(ProjectTag::as_returning())
        .get_result(conn)
}

pub fn insert_many(
    conn: &mut PooledConn,
    project_tags: Vec<ProjectTag>,
) -> Result<Vec<ProjectTag>, Error> {
    diesel::insert_into(projects_tags::table)
        .values(&project_tags)
        .on_conflict((projects_tags::project_id, projects_tags::tag_id))
        .do_nothing()
        // .do_update()
        // .set(projects_tags::sort_order.eq(excluded(projects_tags::sort_order)))
        .returning(ProjectTag::as_returning())
        .get_results(conn)
}

pub fn replace_many(
    conn: &mut PooledConn,
    project_id: i32,
    tags: Vec<ProjectTag>,
) -> Result<Vec<ProjectTag>, Error> {
    conn.transaction(|conn| {
        diesel::delete(
            projects_tags::dsl::projects_tags.filter(projects_tags::project_id.eq(project_id)),
        )
        .execute(conn)?;

        diesel::insert_into(projects_tags::table)
            .values(&tags)
            .returning(ProjectTag::as_returning())
            .get_results(conn)
    })
}

// pub fn delete_by_project_id(
//     conn: &mut PooledConn,
//     project_id: i32,
// ) -> Result<Vec<ProjectTag>, Error> {
//     diesel::delete(
//         projects_tags::dsl::projects_tags
//             .filter(projects_tags::project_id.eq(project_id)),
//     )
//     .returning(ProjectTag::as_returning())
//     .get_results(conn)
// }

// pub fn delete_by_tag_id(conn: &mut PooledConn, tag_id: i32) -> Result<Vec<ProjectTag>, Error> {
//     diesel::delete(
//         projects_tags::dsl::projects_tags.filter(projects_tags::tag_id.eq(tag_id)),
//     )
//     .returning(ProjectTag::as_returning())
//     .get_results(conn)
// }

pub fn delete(
    conn: &mut PooledConn,
    project_id: i32,
    tag_id: i32,
) -> Result<Vec<ProjectTag>, Error> {
    diesel::delete(
        projects_tags::dsl::projects_tags.filter(
            projects_tags::tag_id
                .eq(tag_id)
                .and(projects_tags::project_id.eq(project_id)),
        ),
    )
    .returning(ProjectTag::as_returning())
    .get_results(conn)
}
