use actix_web::{HttpResponse, Responder, delete, post, put, web};

use super::{ExperienceTag, InsertManyExperienceTagsBody, repository};
use crate::{DbPool, config::error_handler::{AppError, ErrorResponse}};

#[utoipa::path(
    tag = "experiences-tags",
    request_body = ExperienceTag,
    responses(
        (status = 201, description = "Experience tag created", body = ExperienceTag),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
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

#[utoipa::path(
    tag = "experiences-tags",
    request_body = InsertManyExperienceTagsBody,
    responses(
        (status = 201, description = "Experience tags created", body = [ExperienceTag]),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
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

#[utoipa::path(
    tag = "experiences-tags",
    request_body = Vec<ExperienceTag>,
    responses(
        (status = 201, description = "Experience tags replaced", body = [ExperienceTag]),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("experience_id" = i32, Path, description = "Experience id")
    ),
    security(("token_jwt" = []))
)]
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

#[utoipa::path(
    tag = "experiences-tags",
    responses(
        (status = 200, description = "Experience tag deleted", body = ExperienceTag),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("experience_id" = i32, Path, description = "Experience id"),
        ("tag_id" = i32, Path, description = "Tag id")
    ),
    security(("token_jwt" = []))
)]
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
