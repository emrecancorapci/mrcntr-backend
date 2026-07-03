use actix_web::{HttpResponse, Responder, delete, post, put, web};

use super::{ExperienceTag, InsertManyExperienceTagsBody, repository};
use crate::{DbPool, config::error_handler::AppError};

#[post("")]
pub async fn insert_one(
    pool: web::Data<DbPool>,
    json: web::Json<ExperienceTag>,
) -> Result<impl Responder, AppError> {
    let exp_tag = json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert_one(&mut conn, exp_tag)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[post("/bulk")]
pub async fn insert_many(
    pool: web::Data<DbPool>,
    json: web::Json<InsertManyExperienceTagsBody>,
) -> Result<impl Responder, AppError> {
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

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert_many(&mut conn, exps_tags)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[put("/experience/{experience_id}")]
pub async fn replace_many_by_experience_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    json: web::Json<Vec<ExperienceTag>>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let exp_tag = json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::replace_many(&mut conn, id, exp_tag)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[delete("/experience/{experience_id}/tag/{tag_id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let (exp_id, tag_id) = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::delete(&mut conn, exp_id, tag_id)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(data))
}
