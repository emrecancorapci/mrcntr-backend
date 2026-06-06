use actix_web::{
    HttpResponse, Responder, delete, error::ErrorInternalServerError, get, patch, post, web,
};
use diesel::Connection;

use super::{ExperienceInsertBody, ExperienceUpdateBody, repository};
use crate::{
    DbPool,
    modules::experiences_tags::{self, ExperienceTag},
};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::many(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))
}

#[get("/{id}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let id = path.into_inner();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::one(&mut conn, &id)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    match data {
        Some(exp) => Ok(HttpResponse::Ok().json(exp)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    experience_json: web::Json<ExperienceInsertBody>,
) -> actix_web::Result<impl Responder> {
    let body = experience_json.into_inner();
    let experience = body.to_new_experience();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        conn.transaction(|t| {
            let exp = repository::insert(&mut *t, experience)?;

            if body.tags.is_none() {
                return Ok::<(), diesel::result::Error>(());
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

            return Ok(());
        })
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match data {
        Ok(exp) => Ok(HttpResponse::Ok().json(exp)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    experience_json: web::Json<ExperienceUpdateBody>,
) -> actix_web::Result<impl Responder> {
    let body = experience_json.into_inner();
    let experience = body.to_update_experience();
    let id = path.into_inner();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        conn.transaction(|t| {
            repository::update(&mut *t, &id, experience)?;

            if body.tags.is_none() {
                return Ok::<(), diesel::result::Error>(());
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
            )?;

            return Ok(());
        })
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match data {
        Ok(exp) => Ok(HttpResponse::Ok().json(exp)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[delete("/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let id = path.into_inner();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::delete(&mut conn, &id)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    match data {
        Some(exp) => Ok(HttpResponse::Ok().json(exp)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
