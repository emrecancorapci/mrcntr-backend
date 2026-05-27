use actix_web::{
    HttpResponse, Responder, delete, error::ErrorInternalServerError, get, patch, post, web,
};

use super::{NewExperience, UpdateExperience, repository};
use crate::DbPool;

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

#[get("/{experience_id}")]
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
    experience_json: web::Json<NewExperience>,
) -> actix_web::Result<impl Responder> {
    let experience = experience_json.into_inner();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::insert(&mut conn, experience)
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match data {
        Ok(exp) => Ok(HttpResponse::Ok().json(exp)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[patch("")]
pub async fn update(
    pool: web::Data<DbPool>,
    experience_json: web::Json<UpdateExperience>,
) -> actix_web::Result<impl Responder> {
    let experience = experience_json.into_inner();
    let id = experience.id.clone();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::update(&mut conn, &id, experience)
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match data {
        Ok(exp) => Ok(HttpResponse::Ok().json(exp)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}


#[delete("/{experience_id}")]
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
