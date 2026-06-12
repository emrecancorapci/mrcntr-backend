use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::repository;
use crate::{
    DbPool,
    config::error_handler::AppError,
    modules::categories::{Category, UpdateCategory},
};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError>  {
    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::many(&mut conn).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Ok().json(data));
}

#[get("/{slug}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError>  {
    let slug = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::one(&mut conn, &slug).map_err(AppError::from)
    })
    .await??;

    let data = result.ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    tag_json: web::Json<Category>,
) -> Result<impl Responder, AppError>  {
    let tag = tag_json.into_inner();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::insert(&mut conn, tag).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Created().json(data));
}

#[patch("/{slug}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    tag_json: web::Json<UpdateCategory>,
) -> Result<impl Responder, AppError>  {
    let tag = tag_json.into_inner();
    let slug = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::update(&mut conn, &slug, &tag.title).map_err(AppError::from)
    })
    .await??;

    let data = result.ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}

#[delete("/{slug}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError>  {
    let slug = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::delete(&mut conn, &slug).map_err(AppError::from)
    })
    .await??;

    let data = result.ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}
