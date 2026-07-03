use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewProjectAiUsage, UpdateProjectAiUsage, repository};
use crate::{DbPool, config::error_handler::AppError};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::many(&mut conn).await.map_err(AppError::from)?;
    Ok(HttpResponse::Ok().json(data))
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
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::one(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("ProjectAIUsage not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewProjectAiUsage>,
) -> Result<impl Responder, AppError> {
    let project_ai_usage = body_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert(&mut conn, project_ai_usage)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<UpdateProjectAiUsage>,
) -> Result<impl Responder, AppError> {
    let project_ai_usage = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::update(&mut conn, id, project_ai_usage)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("ProjectAIUsage not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
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
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::delete(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("ProjectAIUsage not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
