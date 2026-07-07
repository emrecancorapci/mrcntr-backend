use super::{NewProjectTag, ProjectTag};
use crate::{
    PooledConn,
    modules::{projects::Project, tags::Tag},
    schema::projects_tags,
};

use diesel::{
    BelongingToDsl, BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper,
    result::Error, upsert::excluded,
};
use diesel_async::{AsyncConnection, RunQueryDsl};

pub async fn tags_by_project(
    conn: &mut PooledConn,
    project: &Project,
) -> Result<Vec<(ProjectTag, Tag)>, Error> {
    ProjectTag::belonging_to(&project)
        .inner_join(crate::schema::tags::table)
        .select((ProjectTag::as_select(), Tag::as_select()))
        .load::<(ProjectTag, Tag)>(conn)
        .await
}

pub async fn tags_by_projects(
    conn: &mut PooledConn,
    project: &Vec<Project>,
) -> Result<Vec<(ProjectTag, Tag)>, Error> {
    ProjectTag::belonging_to(project)
        .inner_join(crate::schema::tags::table)
        .select((ProjectTag::as_select(), Tag::as_select()))
        .load::<(ProjectTag, Tag)>(conn)
        .await
}

// pub async fn many_by_project_id(
//     conn: &mut PooledConn,
//     project_id: i32,
// ) -> Result<Vec<ProjectTag>, Error> {
//     projects_tags::table
//         .filter(projects_tags::project_id.eq(project_id))
//         .get_results(conn)
// }

// pub async fn many_by_tag_id(conn: &mut PooledConn, tag_id: i32) -> Result<Vec<ProjectTag>, Error> {
//     projects_tags::table
//         .filter(projects_tags::tag_id.eq(tag_id))
//         .get_results(conn)
// }

// pub async fn one(
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

// pub async fn many(conn: &mut PooledConn) -> Result<Vec<ProjectTag>, Error> {
//     projects_tags::table.get_results(conn)
// }

pub async fn insert_one(
    conn: &mut PooledConn,
    project_tag: NewProjectTag,
) -> Result<ProjectTag, Error> {
    diesel::insert_into(projects_tags::table)
        .values(&project_tag)
        .on_conflict((projects_tags::project_id, projects_tags::tag_id))
        .do_update()
        .set((
            projects_tags::sort_order.eq(excluded(projects_tags::sort_order)),
            projects_tags::is_featured.eq(excluded(projects_tags::is_featured)),
        ))
        .returning(ProjectTag::as_returning())
        .get_result(conn)
        .await
}

pub async fn insert_many(
    conn: &mut PooledConn,
    project_tags: Vec<NewProjectTag>,
) -> Result<Vec<ProjectTag>, Error> {
    diesel::insert_into(projects_tags::table)
        .values(&project_tags)
        .on_conflict((projects_tags::project_id, projects_tags::tag_id))
        .do_update()
        .set((
            projects_tags::sort_order.eq(excluded(projects_tags::sort_order)),
            projects_tags::is_featured.eq(excluded(projects_tags::is_featured)),
        ))
        .returning(ProjectTag::as_returning())
        .get_results(conn)
        .await
}

pub async fn replace_many(
    conn: &mut PooledConn,
    project_id: i32,
    tags: Vec<ProjectTag>,
) -> Result<Vec<ProjectTag>, Error> {
    conn.transaction(async |conn| {
        diesel::delete(
            projects_tags::dsl::projects_tags.filter(projects_tags::project_id.eq(project_id)),
        )
        .execute(conn)
        .await?;

        diesel::insert_into(projects_tags::table)
            .values(&tags)
            .returning(ProjectTag::as_returning())
            .get_results(conn)
            .await
    })
    .await
}

// pub async fn delete_by_project_id(
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

// pub async fn delete_by_tag_id(conn: &mut PooledConn, tag_id: i32) -> Result<Vec<ProjectTag>, Error> {
//     diesel::delete(
//         projects_tags::dsl::projects_tags.filter(projects_tags::tag_id.eq(tag_id)),
//     )
//     .returning(ProjectTag::as_returning())
//     .get_results(conn)
// }

pub async fn delete(
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
    .await
}

pub async fn delete_by_project_id(
    conn: &mut PooledConn,
    project_id: i32,
) -> Result<Vec<ProjectTag>, Error> {
    diesel::delete(
        projects_tags::dsl::projects_tags.filter(projects_tags::project_id.eq(project_id)),
    )
    .returning(ProjectTag::as_returning())
    .get_results(conn)
    .await
}
