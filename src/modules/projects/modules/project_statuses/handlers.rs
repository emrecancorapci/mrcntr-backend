use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewProjectStatus, ProjectStatus, UpdateProjectStatus, repository};
use crate::{DbPool, config::error_handler::{AppError, ErrorResponse}};

#[utoipa::path(
    tags = ["Project Status"],
    responses(
        (status = 200, description = "ProjectStatuses", body = Vec<ProjectStatus>),
        (status = 500, body = ErrorResponse)
    )
)]
#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::many(&mut conn).await.map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project Status"],
    responses(
        (status = 200, description = "ProjectStatus", body = ProjectStatus),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(   
        ("id" = i32, description = "ProjectStatus ID")
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
        .ok_or_else(|| AppError::not_found("ProjectStatus not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project Status"],
    responses(
        (status = 201, description = "ProjectStatus created", body = ProjectStatus),
        (status = 500, body = ErrorResponse)
    )
)]
#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewProjectStatus>,
) -> Result<impl Responder, AppError> {
    let project_status = body_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert(&mut conn, project_status)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[utoipa::path(
    tags = ["Project Status"],
    responses(
        (status = 200, description = "ProjectStatus updated", body = ProjectStatus),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(   
        ("id" = i32, description = "ProjectStatus ID")
    )
)]
#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<UpdateProjectStatus>,
) -> Result<impl Responder, AppError> {
    let project_status = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::update(&mut conn, id, project_status)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("ProjectStatus not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project Status"],
    responses(
        (status = 200, description = "ProjectStatus deleted", body = ProjectStatus),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(   
        ("id" = i32, description = "ProjectStatus ID")
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
        .ok_or_else(|| AppError::not_found("ProjectStatus not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
