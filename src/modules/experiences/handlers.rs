use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use diesel_async::AsyncConnection;
use validator::Validate;

use super::{
    ExperienceInsertBody, ExperienceResponse, ExperienceUpdateBody,
    experiences_tags::{self, ExperienceTag},
    repository,
};
use crate::{DbPool, config::error_handler::{AppError, ErrorResponse}, modules::tags};

#[utoipa::path(
    tags = ["Experience"],
    responses(
        (status = 200, description = "List of experiences", body = [ExperienceResponse]),
        (status = 500, body = ErrorResponse)
    ),
    security(())
)]
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

#[utoipa::path(
    tags = ["Experience"],
    responses(
        (status = 200, description = "Experience detail", body = ExperienceResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Experience id")
    ),
    security(())
)]
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

#[utoipa::path(
    tags = ["Experience"],
    request_body = ExperienceInsertBody,
    responses(
        (status = 201, description = "Experience created", body = ExperienceResponse),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    experience_json: web::Json<ExperienceInsertBody>,
) -> Result<impl Responder, AppError> {
    let body = experience_json.into_inner();
    let experience = body.to_new_experience();

    experience.validate()?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = conn
        .transaction(async |t| {
            let mut exp: ExperienceResponse = repository::insert(t, experience).await?.into();

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

            let tags = tags::repository::many_by_ids(t, tag_ids).await?;

            exp.tags = tags;

            Ok::<ExperienceResponse, AppError>(exp)
        })
        .await?;

    Ok(HttpResponse::Created().json(data))
}

#[utoipa::path(
    tags = ["Experience"],
    request_body = ExperienceUpdateBody,
    responses(
        (status = 200, description = "Experience updated", body = ExperienceResponse),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Experience id")
    ),
    security(("token_jwt" = []))
)]
#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    experience_json: web::Json<ExperienceUpdateBody>,
) -> Result<impl Responder, AppError> {
    let body = experience_json.into_inner();
    let experience = body.to_update_experience();
    let id = path.into_inner();

    experience.validate()?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = conn
        .transaction(async |t| {
            let exp = repository::update(&mut *t, &id, experience)
                .await?
                .ok_or_else(|| AppError::not_found("Experience not found".to_string()))?;

            let mut experience: ExperienceResponse = exp.clone().into();

            experience.tags = if body.tags.is_none() {
                let exps_tags = experiences_tags::tags_by_experience(t, &exp).await?;

                exps_tags.into_iter().map(|(_, t)| t).collect()
            } else {
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

                tags::repository::many_by_ids(t, tag_ids).await?
            };

            Ok::<ExperienceResponse, AppError>(experience)
        })
        .await?;

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Experience"],
    responses(
        (status = 200, description = "Experience deleted", body = ExperienceResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Experience id")
    ),
    security(("token_jwt" = []))
)]
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

    let data: ExperienceResponse = repository::delete(&mut conn, &id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Category not found".to_string()))?
        .into();

    Ok(HttpResponse::Ok().json(data))
}
