use super::{NewProjectTag, ProjectTag, TagInsertItem, repository};
use crate::{AppError, DbPool, config::error_handler::ErrorResponse, modules::tags::Tag};

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

// #[post("")]
// pub async fn insert_one(
//     pool: web::Data<DbPool>,
//     path: web::Path<i32>,
//     json: web::Json<NewProjectTagRequest>,
// ) -> Result<impl Responder, AppError> {
//     let project_tag = json.into_inner();
//     let project_id = path.into_inner();

//     let mut conn = pool
//         .get()
//         .await
//         .map_err(|err| AppError::internal(err.to_string()))?;

//     let new_project_tag = NewProjectTag::from_request(project_tag, project_id);

//     let data = repository::insert_one(&mut conn, new_project_tag)
//         .await
//         .map_err(AppError::from)?;

//     Ok(HttpResponse::Created().json(data))
// }

#[utoipa::path(
    tag = "projects",
    responses(
        (status = 200, description = "List of tags for a project", body = [Tag]),
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

    let projects_tags = repository::many_by_project_id(&mut conn, &project_id)
        .await
        .map_err(AppError::from)?;

    let data = projects_tags
        .into_iter()
        .map(|(_, t)| t)
        .collect::<Vec<Tag>>();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tag = "projects",
    request_body = Vec<TagInsertItem>,
    responses(
        (status = 201, description = "Project tags created", body = [ProjectTag]),
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
    json: web::Json<Vec<TagInsertItem>>,
) -> Result<impl Responder, AppError> {
    let project_id = path.into_inner();
    let tags = json.into_inner();
    let exps_tags = tags
        .into_iter()
        .enumerate()
        .map(|(i, item)| NewProjectTag::from_item(item, project_id, i as i16))
        .collect::<Vec<NewProjectTag>>();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert_many(&mut conn, exps_tags)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[utoipa::path(
    tag = "projects",
    request_body = Vec<ProjectTag>,
    responses(
        (status = 201, description = "Project tags replaced", body = [ProjectTag]),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("project_id" = i32, Path, description = "Project id")
    ),
    security(("token_jwt" = []))
)]
#[put("")]
pub async fn replace_many_by_project_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    json: web::Json<Vec<ProjectTag>>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let project_tag = json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::replace_many(&mut conn, id, project_tag)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[utoipa::path(
    tag = "projects",
    responses(
        (status = 200, description = "Project tag deleted", body = ProjectTag),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("project_id" = i32, Path, description = "Project id"),
        ("tag_id" = i32, Path, description = "Tag id")
    ),
    security(("token_jwt" = []))
)]
#[delete("/{tag_id}")]
pub async fn delete_by_project_and_tag_id(
    pool: web::Data<DbPool>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let (project_id, tag_id) = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::delete(&mut conn, project_id, tag_id)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(data))
}
