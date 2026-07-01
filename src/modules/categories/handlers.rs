use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::repository;
use crate::{
    DbPool,
    config::error_handler::AppError,
    modules::categories::{Category, UpdateCategory},
};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::many(&mut conn).await.map_err(AppError::from)?;
    Ok(HttpResponse::Ok().json(data))
}

#[get("/{slug}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let slug = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::one(&mut conn, &slug)
        .await
        .map_err(AppError::from)?;
    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    tag_json: web::Json<Category>,
) -> Result<impl Responder, AppError> {
    let tag = tag_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::insert(&mut conn, tag)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[patch("/{slug}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    tag_json: web::Json<UpdateCategory>,
) -> Result<impl Responder, AppError> {
    let tag = tag_json.into_inner();
    let slug = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::update(&mut conn, &slug, &tag.title)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[delete("/{slug}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let slug = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::delete(&mut conn, &slug)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
