use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{AsyncConnection, RunQueryDsl};

use super::{
    ExperienceInsertBody, ExperienceResponse, ExperienceUpdateBody,
    experiences_tags::{self, ExperienceTag},
    repository,
};
use crate::{DbPool, config::error_handler::AppError, modules::tags::Tag, schema::tags};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let exp = repository::many(&mut conn).await.map_err(AppError::from)?;

    let exps_tags = experiences_tags::tags_by_experiences(&mut conn, &exp).await?;

    let data = exp
        .into_iter()
        .map(|e| ExperienceResponse::from_experience_with_tags(e, &exps_tags))
        .collect::<Vec<ExperienceResponse>>();

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

    let exp = repository::one(&mut conn, &id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Category not found".to_string()))?;

    let exp_tags = experiences_tags::tags_by_experience(&mut conn, &exp).await?;

    let data = ExperienceResponse::from_experience_with_tags(exp, &exp_tags);

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
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = conn
        .transaction(async |t| {
            let exp = repository::insert(t, experience).await?;

            let mut exp: ExperienceResponse = exp.into();

            if body.tags.is_none() {
                return Ok(exp);
            }

            let tag_ids = body.tags.unwrap_or_default();

            if tag_ids.len() == 0 {
                return Ok(exp);
            }

            experiences_tags::repository::insert_many(
                t,
                (&tag_ids)
                    .into_iter()
                    .enumerate()
                    .map(|(i, tag_id)| experiences_tags::ExperienceTag {
                        experience_id: exp.id,
                        tag_id: *tag_id,
                        sort_order: Some(i as i16),
                    })
                    .collect(),
            )
            .await?;

            let tags = tags::table
                .filter(tags::id.eq_any(tag_ids))
                .get_results::<Tag>(t)
                .await?;

            exp.tags = tags;

            Ok::<ExperienceResponse, AppError>(exp)
        })
        .await?;

    Ok(HttpResponse::Created().json(data))
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
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = conn
        .transaction(async |t| {
            let exp = repository::update(&mut *t, &id, experience)
                .await?
                .ok_or_else(|| AppError::not_found("Experience not found".to_string()))?;

            let mut exp: ExperienceResponse = exp.into();

            if body.tags.is_none() {
                // TODO: Import existing tags

                return Ok::<ExperienceResponse, AppError>(exp);
            }

            let tag_ids = body.tags.unwrap_or_default();

            experiences_tags::repository::replace_many(
                &mut *t,
                id,
                (&tag_ids)
                    .into_iter()
                    .enumerate()
                    .map(|(i, tag_id)| ExperienceTag {
                        experience_id: id,
                        tag_id: *tag_id,
                        sort_order: Some(i as i16),
                    })
                    .collect(),
            )
            .await
            .map_err(AppError::from)?;

            let tags = tags::table
                .filter(tags::id.eq_any(tag_ids))
                .get_results::<Tag>(t)
                .await?;

            exp.tags = tags;

            Ok::<ExperienceResponse, AppError>(exp)
        })
        .await?;

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

    let data = repository::delete(&mut conn, &id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Category not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
