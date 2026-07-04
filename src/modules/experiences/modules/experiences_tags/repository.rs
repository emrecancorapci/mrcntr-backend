use diesel::{
    BelongingToDsl, BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper,
    result::Error, upsert::excluded,
};
use diesel_async::{AsyncConnection, RunQueryDsl};

use super::ExperienceTag;
use crate::{
    PooledConn,
    modules::{experiences::Experience, tags::Tag},
    schema::experiences_tags,
};

pub async fn tags_by_experience(
    conn: &mut PooledConn,
    exp: &Experience,
) -> Result<Vec<(ExperienceTag, Tag)>, Error> {
    ExperienceTag::belonging_to(&exp)
        .inner_join(crate::schema::tags::table)
        .select((ExperienceTag::as_select(), Tag::as_select()))
        .load::<(ExperienceTag, Tag)>(conn)
        .await
}

pub async fn tags_by_experiences(
    conn: &mut PooledConn,
    exp: &Vec<Experience>,
) -> Result<Vec<(ExperienceTag, Tag)>, Error> {
    ExperienceTag::belonging_to(exp)
        .inner_join(crate::schema::tags::table)
        .select((ExperienceTag::as_select(), Tag::as_select()))
        .load::<(ExperienceTag, Tag)>(conn)
        .await
}

// pub async fn many_by_tag_id(conn: &mut PooledConn, tag_id: i32) -> Result<Vec<ExperienceTag>, Error> {
//     experiences_tags::table
//         .filter(experiences_tags::tag_id.eq(tag_id))
//         .get_results(conn)
// }

// pub async fn one(
//     conn: &mut PooledConn,
//     tag_id: i32,
//     experience_id: i32,
// ) -> Result<Option<ExperienceTag>, Error> {
//     experiences_tags::table
//         .filter(experiences_tags::tag_id.eq(tag_id))
//         .filter(experiences_tags::experience_id.eq(experience_id))
//         .get_result(conn)
//         .optional()
// }

// pub async fn many(conn: &mut PooledConn) -> Result<Vec<ExperienceTag>, Error> {
//     experiences_tags::table.get_results(conn)
// }

pub async fn insert_one(
    conn: &mut PooledConn,
    experience: ExperienceTag,
) -> Result<ExperienceTag, Error> {
    diesel::insert_into(experiences_tags::table)
        .values(&experience)
        .on_conflict((experiences_tags::experience_id, experiences_tags::tag_id))
        .do_update()
        .set(experiences_tags::sort_order.eq(excluded(experiences_tags::sort_order)))
        .returning(ExperienceTag::as_returning())
        .get_result(conn)
        .await
}

pub async fn insert_many(
    conn: &mut PooledConn,
    experience: Vec<ExperienceTag>,
) -> Result<Vec<ExperienceTag>, Error> {
    diesel::insert_into(experiences_tags::table)
        .values(&experience)
        .on_conflict((experiences_tags::experience_id, experiences_tags::tag_id))
        .do_update()
        .set(experiences_tags::sort_order.eq(excluded(experiences_tags::sort_order)))
        .returning(ExperienceTag::as_returning())
        .get_results(conn)
        .await
}

pub async fn replace_many(
    conn: &mut PooledConn,
    experience_id: i32,
    tags: Vec<ExperienceTag>,
) -> Result<Vec<ExperienceTag>, Error> {
    conn.transaction(async |conn| {
        diesel::delete(
            experiences_tags::dsl::experiences_tags
                .filter(experiences_tags::experience_id.eq(experience_id)),
        )
        .execute(conn)
        .await?;

        diesel::insert_into(experiences_tags::table)
            .values(&tags)
            .returning(ExperienceTag::as_returning())
            .get_results(conn)
            .await
    })
    .await
}

// pub async fn delete_by_experience_id(
//     conn: &mut PooledConn,
//     experience_id: i32,
// ) -> Result<Vec<ExperienceTag>, Error> {
//     diesel::delete(
//         experiences_tags::dsl::experiences_tags
//             .filter(experiences_tags::experience_id.eq(experience_id)),
//     )
//     .returning(ExperienceTag::as_returning())
//     .get_results(conn)
// }

// pub async fn delete_by_tag_id(conn: &mut PooledConn, tag_id: i32) -> Result<Vec<ExperienceTag>, Error> {
//     diesel::delete(
//         experiences_tags::dsl::experiences_tags.filter(experiences_tags::tag_id.eq(tag_id)),
//     )
//     .returning(ExperienceTag::as_returning())
//     .get_results(conn)
// }

pub async fn delete(
    conn: &mut PooledConn,
    experience_id: i32,
    tag_id: i32,
) -> Result<Vec<ExperienceTag>, Error> {
    diesel::delete(
        experiences_tags::dsl::experiences_tags.filter(
            experiences_tags::tag_id
                .eq(tag_id)
                .and(experiences_tags::experience_id.eq(experience_id)),
        ),
    )
    .returning(ExperienceTag::as_returning())
    .get_results(conn)
    .await
}
