use super::{NewProject, NewProjectRequest, ProjectResponse, UpdateProject, repository, submodules::tags};
use crate::{
    DbPool,
    config::error_handler::AppError,
    modules::{project_blocks, project_links, tags::Tag},
};

use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel_async::AsyncConnection;
use validator::Validate;

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

    let (p, p_s, p_t, p_ai) = repository::one(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Project not found".to_string()))?;

    let blocks = project_blocks::repository::many_by_project(&mut conn, &p).await?;
    let links = project_links::repository::many_by_project(&mut conn, &p).await?;

    let project_tags = tags::repository::tags_by_project(&mut conn, &p).await?;
    let tags = project_tags
        .into_iter()
        .map(|(_, t)| t)
        .collect::<Vec<Tag>>();

    let data = ProjectResponse::from_complete(p, p_s, p_t, p_ai, blocks, links, tags);

    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<NewProjectRequest>,
) -> Result<impl Responder, AppError> {
    let project_request = body_json.into_inner();

    project_request.validate()?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = conn
        .transaction(async |t| {
            let new_project: NewProject = project_request.clone().into();
            let id = repository::insert(t, new_project).await?.id;

            let new_blocks: Vec<project_blocks::NewProjectBlock> = project_request
                .project_blocks
                .into_iter()
                .map(|b| project_blocks::NewProjectBlock {
                    project_id: id,
                    sort_order: b.sort_order,
                    title: b.title,
                    content: b.content,
                    is_active: b.is_active,
                })
                .collect();

            let new_links: Vec<project_links::NewProjectLink> = project_request
                .project_links
                .into_iter()
                .map(|l| project_links::NewProjectLink {
                    project_id: id,
                    sort_order: l.sort_order,
                    title: l.title,
                    link: l.link,
                })
                .collect();

            let blocks = project_blocks::repository::insert(t, new_blocks).await?;
            let links = project_links::repository::insert(t, new_links).await?;

            let (p, p_s, p_t, p_ai) = repository::one(t, id)
                .await?
                .ok_or_else(|| AppError::not_found("Project not found".to_string()))?;

            let project_tags = tags::repository::tags_by_project(t, &p).await?;
            let tags = project_tags
                .into_iter()
                .map(|(_, t)| t)
                .collect::<Vec<Tag>>();

            let project_response =
                ProjectResponse::from_complete(p, p_s, p_t, p_ai, blocks, links, tags);

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

    project.validate()?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: ProjectResponse = repository::update(&mut conn, id, project)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Project not found".to_string()))?
        .into();

    // TODO: Updating sub modules

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

    let data = conn
        .transaction(async |t| {
            let project = repository::delete(t, id)
                .await
                .map_err(AppError::from)?
                .ok_or_else(|| AppError::not_found("Project not found".to_string()))?;

            project_blocks::repository::delete_by_project_id(t, id)
                .await
                .map_err(AppError::from)?;
            project_links::repository::delete_by_project_id(t, id)
                .await
                .map_err(AppError::from)?;
            tags::repository::delete_by_project_id(t, id)
                .await
                .map_err(AppError::from)?;

            let project =
                ProjectResponse::from_complete(project, None, None, None, vec![], vec![], vec![]);

            return Ok::<ProjectResponse, AppError>(project);
        })
        .await?;

    Ok(HttpResponse::Ok().json(data))
}
