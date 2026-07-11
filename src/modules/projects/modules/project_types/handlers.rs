use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewProjectType, ProjectType, UpdateProjectType, repository};
use crate::{DbPool, config::error_handler::{AppError, ErrorResponse}};

#[utoipa::path(
    tags = ["Project Type"],
    responses(
        (status = 200, description = "ProjectTypes", body = Vec<ProjectType>),
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
    tags = ["Project Type"],
    responses(
        (status = 200, description = "ProjectType", body = ProjectType),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(   
        ("id" = i32, description = "ProjectType ID")
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
        .ok_or_else(|| AppError::not_found("ProjectType not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project Type"],
    responses(
        (status = 201, description = "ProjectType created", body = ProjectType),
        (status = 500, body = ErrorResponse)
    )
)]
#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewProjectType>,
) -> Result<impl Responder, AppError> {
    let project_type = body_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert(&mut conn, project_type)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[utoipa::path(
    tags = ["Project Type"],
    responses(
        (status = 200, description = "ProjectType updated", body = ProjectType),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(   
        ("id" = i32, description = "ProjectType ID")
    )
)]
#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<UpdateProjectType>,
) -> Result<impl Responder, AppError> {
    let project_type = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::update(&mut conn, id, project_type)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("ProjectType not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project Type"],
    responses(
        (status = 200, description = "ProjectType deleted", body = ProjectType),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(   
        ("id" = i32, description = "ProjectType ID")
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
        .ok_or_else(|| AppError::not_found("ProjectType not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
