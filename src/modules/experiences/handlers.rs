use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel::Connection;

use super::{ExperienceInsertBody, ExperienceUpdateBody, repository};
use crate::{
    DbPool,
    config::error_handler::AppError,
    modules::experiences_tags::{self, ExperienceTag},
};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError>  {
    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::many(&mut conn).map_err(AppError::from)
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

#[get("/{id}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError>  {
    let id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::one(&mut conn, &id).map_err(AppError::from)
    })
    .await??;

    let data = result.ok_or_else(|| AppError::NotFound("Experience not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    experience_json: web::Json<ExperienceInsertBody>,
) -> Result<impl Responder, AppError>  {
    let body = experience_json.into_inner();
    let experience = body.to_new_experience();

    web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        conn.transaction(|t| {
            let exp = repository::insert(&mut *t, experience)?;

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
            )?;

            Ok(())
        })
    })
    .await??;

    Ok(HttpResponse::Created().json(()))
}

#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    experience_json: web::Json<ExperienceUpdateBody>,
) -> Result<impl Responder, AppError>  {
    let body = experience_json.into_inner();
    let experience = body.to_update_experience();
    let id = path.into_inner();

    web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        conn.transaction(|t| {
            repository::update(&mut *t, &id, experience)?
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
            .map_err(AppError::from)?;

            Ok(())
        })
    })
    .await??;

    Ok(HttpResponse::Ok().json(()))
}

#[delete("/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError>  {
    let id = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::delete(&mut conn, &id).map_err(AppError::from)
    })
    .await??;

    let data = result.ok_or_else(|| AppError::NotFound("Experience not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
