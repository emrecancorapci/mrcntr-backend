use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewBlogpost, UpdateBlogpost, repository};
use crate::{
    DbPool,
    config::error_handler::{AppError, ErrorResponse},
    modules::blogposts::Blogpost,
};

#[utoipa::path(
    tag = "blogposts",
    responses(
        (status = 200, description = "List of blogposts", body = [Blogpost]),
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

    let data = repository::many(&mut conn).await.map_err(AppError::from)?;

    return Ok(HttpResponse::Ok().json(data));
}

#[utoipa::path(
    tag = "blogposts",
    responses(
        (status = 200, description = "Blogpost detail", body = Blogpost),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Blogpost id")
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

    let data = repository::one(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Blogpost not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}

#[utoipa::path(
    tag = "blogposts",
    request_body = NewBlogpost,
    responses(
        (status = 201, description = "Blogpost created", body = Blogpost),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewBlogpost>,
) -> Result<impl Responder, AppError> {
    let blogpost = body_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert(&mut conn, blogpost)
        .await
        .map_err(AppError::from)?;

    return Ok(HttpResponse::Created().json(data));
}

#[utoipa::path(
    tag = "blogposts",
    request_body = UpdateBlogpost,
    responses(
        (status = 200, description = "Blogpost updated", body = Blogpost),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Blogpost id")
    ),
    security(("token_jwt" = []))
)]
#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<UpdateBlogpost>,
) -> Result<impl Responder, AppError> {
    let blogpost = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::update(&mut conn, id, blogpost)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Blogpost not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}

#[utoipa::path(
    tag = "blogposts",
    responses(
        (status = 200, description = "Blogpost deleted", body = Blogpost),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Blogpost id")
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

    let data = repository::delete(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Blogpost not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}
