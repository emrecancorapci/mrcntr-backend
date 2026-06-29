use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewComment, UpdateComment, repository};
use crate::{DbPool, config::error_handler::AppError, modules};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::many(&mut conn).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Ok().json(data));
}

#[get("/{id}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::one(&mut conn, id).map_err(AppError::from)
    })
    .await??;

    let data = result.ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewComment>,
) -> Result<impl Responder, AppError> {
    let comment = body_json.into_inner();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        let blogpost = modules::blogposts::repository::one(&mut conn, comment.blogpost_id)
            .map_err(AppError::from)?;

        if blogpost.is_none() {
            return Err(AppError::BadRequest("Blogpost not found".to_string()));
        }

        repository::insert(&mut conn, comment).map_err(AppError::from)
    })
    .await??;

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

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::update(&mut conn, id, comment).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Ok().json(data));
}

#[delete("/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::delete(&mut conn, id).map_err(AppError::from)
    })
    .await??;

    let data = result.ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}
