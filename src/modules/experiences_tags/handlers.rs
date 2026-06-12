use actix_web::{HttpResponse, Responder, delete, post, put, web};

use super::{ExperienceTag, repository};
use crate::{
    DbPool, config::error_handler::AppError,
    modules::experiences_tags::InsertManyExperienceTagsBody,
};

#[post("")]
pub async fn insert_one(
    pool: web::Data<DbPool>,
    json: web::Json<ExperienceTag>,
) -> Result<impl Responder, AppError>  {
    let exp_tag = json.into_inner();
    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::insert_one(&mut conn, exp_tag).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Created().json(data));
}

#[post("/bulk")]
pub async fn insert_many(
    pool: web::Data<DbPool>,
    json: web::Json<InsertManyExperienceTagsBody>,
) -> Result<impl Responder, AppError>  {
    let body = json.into_inner();
    let exps_tags = body
        .tags
        .into_iter()
        .map(|tag| ExperienceTag {
            tag_id: tag.tag_id,
            experience_id: body.experience_id,
            sort_order: tag.sort,
        })
        .collect::<Vec<ExperienceTag>>();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::insert_many(&mut conn, exps_tags).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Created().json(data));
}

#[put("/experience/{experience_id}")]
pub async fn replace_many_by_experience_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    json: web::Json<Vec<ExperienceTag>>,
) -> Result<impl Responder, AppError>  {
    let id = path.into_inner();
    let exp_tag = json.into_inner();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::replace_many(&mut conn, id, exp_tag).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Created().json(data));
}

#[delete("/experience/{experience_id}/tag/{tag_id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError>  {
    let (exp_id, tag_id) = path.into_inner();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::delete(&mut conn, exp_id, tag_id).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Ok().json(data));
}
