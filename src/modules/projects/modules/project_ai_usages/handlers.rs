use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewProjectAiUsage, ProjectAiUsage, UpdateProjectAiUsage, repository};
use crate::{AppError, DbPool, config::error_handler::ErrorResponse};

#[utoipa::path(
    tags = ["Project AI Usage"],
    responses(
        (status = 200, description = "ProjectAIUsage found from database", body=Vec<ProjectAiUsage>),
        (status = 500, body=ErrorResponse)
    ),
    params()
)]
#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: Vec<ProjectAiUsage> = repository::many(&mut conn).await.map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project AI Usage"],
    responses(
        (status = 200, description = "ProjectAIUsage found from database", body=Vec<ProjectAiUsage>),
        (status = 500, body=ErrorResponse)
    ),
    params(
        ("id", description = "Project AI Usage id")
    )
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
        .ok_or_else(|| AppError::not_found("ProjectAIUsage not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project AI Usage"],
    responses(
        (status = 201, description = "ProjectAIUsage created", body = ProjectAiUsage),
        (status = 500, body = ErrorResponse)
    ),
    params()
)]
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

#[utoipa::path(
    tags = ["Project AI Usage"],
    responses(
        (status = 200, description = "ProjectAIUsage updated", body = ProjectAiUsage),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id", description = "Project AI Usage id")
    )
)]
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

#[utoipa::path(
    tags = ["Project AI Usage"],
    responses(
        (status = 200, description = "ProjectAIUsage deleted", body = ProjectAiUsage),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id", description = "Project AI Usage id")
    )
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
        .ok_or_else(|| AppError::not_found("ProjectAIUsage not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
