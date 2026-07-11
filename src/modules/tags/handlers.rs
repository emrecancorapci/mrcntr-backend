use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use validator::Validate;

use super::{UpdateTag, repository};
use crate::{
    DbPool,
    config::error_handler::{AppError, ErrorResponse},
    modules::tags::{NewTag, TagResponse},
};

#[utoipa::path(
    tags = ["Tag"],
    responses(
        (status = 200, description = "List of tags", body = [TagResponse]),
        (status = 500, body = ErrorResponse)
    ),
    security(())
)]
#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::many(&mut conn)
        .await
        .map_err(AppError::from)?
        .into_iter()
        .map(|t| t.into())
        .collect::<Vec<TagResponse>>();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Tag"],
    responses(
        (status = 200, description = "Tag detail", body = TagResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Tag id")
    ),
    security(())
)]
#[get("/{id}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: TagResponse = repository::one(&mut conn, &id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Tag not found".to_string()))?
        .into();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Tag"],
    request_body = NewTag,
    responses(
        (status = 201, description = "Tag created", body = TagResponse),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    tag_json: web::Json<NewTag>,
) -> Result<impl Responder, AppError> {
    let tag = tag_json.into_inner();

    tag.validate()?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: TagResponse = repository::insert(&mut conn, tag)
        .await
        .map_err(AppError::from)?
        .into();

    Ok(HttpResponse::Created().json(data))
}

#[utoipa::path(
    tags = ["Tag"],
    request_body = UpdateTag,
    responses(
        (status = 200, description = "Tag updated", body = TagResponse),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Tag id")
    ),
    security(("token_jwt" = []))
)]
#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    tag_json: web::Json<UpdateTag>,
) -> Result<impl Responder, AppError> {
    let tag = tag_json.into_inner();
    let id = path.into_inner();

    tag.validate()?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: TagResponse = repository::update(&mut conn, &id, tag)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Tag not found".to_string()))?
        .into();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Tag"],
    responses(
        (status = 200, description = "Tag deleted", body = TagResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Tag id")
    ),
    security(("token_jwt" = []))
)]
#[delete("/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: TagResponse = repository::delete(&mut conn, &id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Tag not found".to_string()))?
        .into();

    Ok(HttpResponse::Ok().json(data))
}
