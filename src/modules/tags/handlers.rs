use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{UpdateTag, repository};
use crate::{DbPool, config::error_handler::AppError, modules::tags::NewTag};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError>  {
    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::many(&mut conn).map_err(AppError::from)
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

#[get("/{id}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError>  {
    let id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::one(&mut conn, &id).map_err(AppError::from)
    })
    .await??;

    let data = result.ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    tag_json: web::Json<NewTag>,
) -> Result<impl Responder, AppError>  {
    let tag = tag_json.into_inner();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::insert(&mut conn, tag).map_err(AppError::from)
    })
    .await??;

    Ok(HttpResponse::Created().json(data))
}

#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    tag_json: web::Json<UpdateTag>,
) -> Result<impl Responder, AppError>  {
    let tag = tag_json.into_inner();
    let id = path.into_inner();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::update(&mut conn, &id, tag).map_err(AppError::from)
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

#[delete("/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError>  {
    let id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::delete(&mut conn, &id).map_err(AppError::from)
    })
    .await??;


    let data = result.ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
