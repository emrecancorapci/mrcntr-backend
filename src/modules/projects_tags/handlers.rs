use actix_web::{HttpResponse, Responder, delete, post, put, web};

use super::{InsertManyProjectTagsBody, ProjectTag, repository};
use crate::{DbPool, config::error_handler::AppError};

#[post("")]
pub async fn insert_one(
    pool: web::Data<DbPool>,
    json: web::Json<ProjectTag>,
) -> Result<impl Responder, AppError> {
    let project_tag = json.into_inner();
    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::insert_one(&mut conn, project_tag).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Created().json(data));
}

#[post("/bulk")]
pub async fn insert_many(
    pool: web::Data<DbPool>,
    json: web::Json<InsertManyProjectTagsBody>,
) -> Result<impl Responder, AppError> {
    let body = json.into_inner();
    let exps_tags = body
        .tags
        .into_iter()
        .map(|tag_id| ProjectTag {
            tag_id,
            project_id: body.project_id,
            // sort_order: tag.sort,
        })
        .collect::<Vec<ProjectTag>>();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::insert_many(&mut conn, exps_tags).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Created().json(data));
}

#[put("/project/{project_id}")]
pub async fn replace_many_by_project_id(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    json: web::Json<Vec<ProjectTag>>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();
    let project_tag = json.into_inner();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::replace_many(&mut conn, id, project_tag).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Created().json(data));
}

#[delete("/project/{project_id}/tag/{tag_id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let (project_id, tag_id) = path.into_inner();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::delete(&mut conn, project_id, tag_id).map_err(AppError::from)
    })
    .await??;

    return Ok(HttpResponse::Ok().json(data));
}
