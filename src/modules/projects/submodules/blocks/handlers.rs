use actix_web::{HttpResponse, Responder, delete, get, post, web};

use crate::{AppError, DbPool, config::error_handler::ErrorResponse, modules::project_blocks::{NewProjectBlock, NewProjectBlockRequest, ProjectBlock, repository}};

#[utoipa::path(
    tags = ["Project"],
    responses(
        (status = 200, description = "List of project blocks", body = [ProjectBlock]),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("project_id" = i32, Path, description = "Project id")
    ),
    security(("token_jwt" = []))
)]
#[get("")]
pub async fn many_by_project_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let project_id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::many_by_project_id(&mut conn, &project_id)
        .await
        .map_err(AppError::from)?;
    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Project"],
    request_body = NewProjectBlockRequest,
    responses(
        (status = 201, description = "Project block created", body = ProjectBlock),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("project_id" = i32, Path, description = "Project id")
    ),
    security(("token_jwt" = []))
)]
#[post("")]
pub async fn insert_by_project_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<NewProjectBlockRequest>,
) -> Result<impl Responder, AppError> {
    let project_block = body_json.into_inner();
    let project_id = path.into_inner();

    let project_block = NewProjectBlock::from_request(project_block, project_id);

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
    tags = ["Project"],
    responses(
        (status = 200, description = "Project blocks deleted", body = [ProjectBlock]),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("project_id" = i32, Path, description = "Project id")
    ),
    security(("token_jwt" = []))
)]
#[delete("")]
pub async fn delete_many_by_project_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let project_id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::delete_by_project_id(&mut conn, project_id)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(data))
}
