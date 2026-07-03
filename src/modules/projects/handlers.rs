use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel_async::AsyncConnection;

use super::{
    NewProject, NewProjectRequest, ProjectResponse, UpdateProject,
    modules::{project_blocks, project_links},
    repository,
};
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
        .ok_or_else(|| AppError::not_found("Project not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewProjectRequest>,
) -> Result<impl Responder, AppError> {
    let project_request = body_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = conn
        .transaction(async |t| {
            let new_project: NewProject = project_request.clone().into();
            let id = repository::insert(t, new_project).await?.id;

            let blocks: Vec<project_blocks::NewProjectBlock> = project_request
                .project_blocks
                .into_iter()
                .map(|b| project_blocks::NewProjectBlock {
                    project_id: id,
                    sort_order: b.sort_order,
                    title: b.title,
                    content: b.content,
                })
                .collect();

            let links: Vec<project_links::NewProjectLink> = project_request
                .project_links
                .into_iter()
                .map(|l| project_links::NewProjectLink {
                    project_id: id,
                    sort_order: l.sort_order,
                    title: l.title,
                    link: l.link,
                })
                .collect();

            project_blocks::repository::insert(t, blocks).await?;
            project_links::repository::insert(t, links).await?;

            let project_response = repository::one(t, id)
                .await?
                .ok_or_else(|| AppError::not_found("Project not found".to_string()))?;

            Ok::<ProjectResponse, AppError>(project_response)
        })
        .await?;

    Ok(HttpResponse::Created().json(data))
}

#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<UpdateProject>,
) -> Result<impl Responder, AppError> {
    let project = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::update(&mut conn, id, project)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Project not found".to_string()))?;

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
        .ok_or_else(|| AppError::not_found("Project not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
