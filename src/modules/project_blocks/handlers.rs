use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewProjectBlock, ProjectBlock, UpdateProjectBlock, repository};
use crate::{
    DbPool,
    config::error_handler::{AppError, ErrorResponse},
};

#[utoipa::path(
    tag = "project_blocks",
    responses(
        (status = 200, description = "ProjectBlocks", body = Vec<ProjectBlock>),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Project id")
    ),
    security(("token_jwt" = []))
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
    tag = "project_blocks",
    responses(
        (status = 200, description = "ProjectBlock", body = ProjectBlock),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Project id"),
        ("id" = i32, Path, description = "ProjectBlock id")
    ),
    security(("token_jwt" = []))
)]
#[get("/{id}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let (_project_id, id) = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::one(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("ProjectBlock not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tag = "project_blocks",
    request_body = NewProjectBlock,
    responses(
        (status = 201, description = "ProjectBlock created", body = ProjectBlock),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Project id")
    ),
    security(("token_jwt" = []))
)]
#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewProjectBlock>,
) -> Result<impl Responder, AppError> {
    let project_block = body_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert(&mut conn, vec![project_block])
        .await
        .map_err(AppError::from)?;

    match data.into_iter().next() {
        Some(created_block) => Ok(HttpResponse::Created().json(created_block)),
        None => Err(AppError::internal("Error while inserting".to_string())),
    }
}

#[utoipa::path(
    tag = "project_blocks",
    request_body = UpdateProjectBlock,
    responses(
        (status = 200, description = "ProjectBlock updated", body = ProjectBlock),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "ProjectBlock id")
    ),
    security(("token_jwt" = []))
)]
#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<UpdateProjectBlock>,
) -> Result<impl Responder, AppError> {
    let project_block = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::update(&mut conn, id, project_block)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("ProjectBlock not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tag = "project_blocks",
    responses(
        (status = 200, description = "ProjectBlock deleted", body = ProjectBlock),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "ProjectBlock id")
    ),
    security(("token_jwt" = []))
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
        .ok_or_else(|| AppError::not_found("ProjectBlock not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
