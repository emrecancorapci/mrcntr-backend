use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel_async::AsyncConnection;

use super::{
    ExperienceInsertBody, ExperienceUpdateBody,
    experiences_tags::{self, ExperienceTag},
    repository,
};
use crate::{DbPool, config::error_handler::AppError};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

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
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::one(&mut conn, &id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    experience_json: web::Json<ExperienceInsertBody>,
) -> Result<impl Responder, AppError> {
    let body = experience_json.into_inner();
    let experience = body.to_new_experience();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    conn.transaction(async |t| {
        let exp = repository::insert(&mut *t, experience).await?;

        if body.tags.is_none() {
            return Ok::<(), AppError>(());
        }

        experiences_tags::repository::insert_many(
            &mut *t,
            body.tags
                .unwrap_or_default()
                .into_iter()
                .enumerate()
                .map(|(i, tag_id)| ExperienceTag {
                    experience_id: exp.id,
                    tag_id,
                    sort_order: Some(i as i16),
                })
                .collect(),
        )
        .await?;

        Ok(())
    });

    Ok(HttpResponse::Created().json(()))
}

#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    experience_json: web::Json<ExperienceUpdateBody>,
) -> Result<impl Responder, AppError> {
    let body = experience_json.into_inner();
    let experience = body.to_update_experience();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    conn.transaction(async |t| {
        repository::update(&mut *t, &id, experience)
            .await?
            .ok_or_else(|| AppError::NotFound("Experience not found".to_string()))?;

        if body.tags.is_none() {
            return Ok::<(), AppError>(());
        }

        experiences_tags::repository::replace_many(
            &mut *t,
            id,
            body.tags
                .unwrap_or_default()
                .into_iter()
                .enumerate()
                .map(|(i, tag_id)| ExperienceTag {
                    experience_id: id,
                    tag_id,
                    sort_order: Some(i as i16),
                })
                .collect(),
        )
        .await
        .map_err(AppError::from)?;

        Ok(())
    });

    Ok(HttpResponse::Ok().json(()))
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
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::delete(&mut conn, &id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
