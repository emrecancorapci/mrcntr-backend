use actix_web::{HttpResponse, Responder, delete, get, post, web};

use crate::{
    AppError, DbPool,
    config::error_handler::ErrorResponse,
    modules::project_links::{NewProjectLink, NewProjectLinkRequest, ProjectLink, repository},
};

#[utoipa::path(
    tag = "projects",
    responses(
        (status = 200, description = "List of project links", body = [ProjectLink]),
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
    tag = "projects",
    request_body = NewProjectLinkRequest,
    responses(
        (status = 201, description = "Project link created", body = ProjectLink),
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
    body_json: web::Json<NewProjectLinkRequest>,
) -> Result<impl Responder, AppError> {
    let project_block = body_json.into_inner();
    let project_id = path.into_inner();

    let project_block = NewProjectLink::from_request(project_block, project_id);

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
    tag = "projects",
    responses(
        (status = 200, description = "Project links deleted", body = [ProjectLink]),
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
