use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewProjectLink, ProjectLink, UpdateProjectLink, repository};
use crate::{DbPool, config::error_handler::{AppError, ErrorResponse}};

#[utoipa::path(
    tags = ["Project Link"],
    responses(
        (status = 200, description = "ProjectLinks", body = Vec<ProjectLink>),
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
    tags = ["Project Link"],
    responses(
        (status = 200, description = "ProjectLink", body = ProjectLink),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(   
        ("id" = i32, description = "ProjectLink ID")
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
        .ok_or_else(|| AppError::not_found("ProjectLink not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project Link"],
    responses(
        (status = 201, description = "ProjectLink created", body = ProjectLink),
        (status = 500, body = ErrorResponse)
    )
)]
#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewProjectLink>,
) -> Result<impl Responder, AppError> {
    let project_link = body_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert(&mut conn, vec![project_link])
        .await
        .map_err(AppError::from)?;

    match data.into_iter().next() {
        Some(created_link) => Ok(HttpResponse::Created().json(created_link)),
        None => Err(AppError::internal("Error while inserting".to_string())),
    }
}

#[utoipa::path(
    tags = ["Project Link"],
    responses(
        (status = 200, description = "ProjectLink updated", body = ProjectLink),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(   
        ("id" = i32, description = "ProjectLink ID")
    )
)]
#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<UpdateProjectLink>,
) -> Result<impl Responder, AppError> {
    let project_link = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::update(&mut conn, id, project_link)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("ProjectLink not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project Link"],
    responses(
        (status = 200, description = "ProjectLink deleted", body = ProjectLink),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(   
        ("id" = i32, description = "ProjectLink ID")
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
        .ok_or_else(|| AppError::not_found("ProjectLink not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
