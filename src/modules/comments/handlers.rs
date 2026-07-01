use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewComment, UpdateComment, repository};
use crate::{DbPool, config::error_handler::AppError, modules};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::many(&mut conn).await.map_err(AppError::from)?;

    return Ok(HttpResponse::Ok().json(data));
}

#[get("/{id}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::one(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewComment>,
) -> Result<impl Responder, AppError> {
    let comment = body_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    // Check blogpost exist
    let _ = modules::blogposts::repository::one(&mut conn, comment.blogpost_id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("Blogpost not found".to_string()))?;

    let data = repository::insert(&mut conn, comment)
        .await
        .map_err(AppError::from)?;

    return Ok(HttpResponse::Created().json(data));
}

#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<UpdateComment>,
) -> Result<impl Responder, AppError> {
    let comment = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::update(&mut conn, id, comment)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}

#[delete("/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::delete(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}
