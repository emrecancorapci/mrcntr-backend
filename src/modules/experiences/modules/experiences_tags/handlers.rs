use actix_web::{HttpResponse, Responder, delete, post, put, web};

use super::{ExperienceTag, repository};
use crate::{
    DbPool,
    config::error_handler::{AppError, ErrorResponse},
    modules::experiences::experiences_tags::InsertManyExperienceTagsItem,
};

#[utoipa::path(
    tags = ["Experience"],
    request_body = Vec<InsertManyExperienceTagsItem>,
    responses(
        (status = 201, description = "Experience tags created", body = [ExperienceTag]),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
#[post("/bulk")]
pub async fn insert_many(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    json: web::Json<Vec<InsertManyExperienceTagsItem>>,
) -> Result<impl Responder, AppError> {
    let tags = json.into_inner();
    let experience_id = path.into_inner();

    let exps_tags = tags
        .into_iter()
        .map(|tag| ExperienceTag {
            experience_id,
            tag_id: tag.tag_id,
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
    tags = ["Experience"],
    request_body = Vec<InsertManyExperienceTagsItem>,
    responses(
        (status = 201, description = "Experience tags replaced", body = [ExperienceTag]),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("experience_id" = i32, Path, description = "Experience id")
    ),
    security(("token_jwt" = []))
)]
#[put("")]
pub async fn replace_many_by_experience_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    json: web::Json<Vec<InsertManyExperienceTagsItem>>,
) -> Result<impl Responder, AppError> {
    let experience_id = path.into_inner();
    let tags = json.into_inner();

    let exps_tags = tags
        .into_iter()
        .map(|tag| ExperienceTag {
            experience_id,
            tag_id: tag.tag_id,
            sort_order: tag.sort,
        })
        .collect::<Vec<ExperienceTag>>();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::replace_many(&mut conn, experience_id, exps_tags)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[utoipa::path(
    tags = ["Experience"],
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
#[delete("/{tag_id}")]
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
