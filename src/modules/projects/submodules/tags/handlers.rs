use super::{NewProjectTag, ProjectTag, TagInsertItem, repository};
use crate::{AppError, DbPool};

use actix_web::{HttpResponse, Responder, delete, post, put, web};

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

#[post("")]
pub async fn insert(
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

#[delete("/{tag_id}")]
pub async fn delete(
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
