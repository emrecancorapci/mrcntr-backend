use actix_web::{
    HttpResponse, Responder, delete, error::ErrorInternalServerError, get, patch, post, web,
};

use super::{Tag, UpdateTag, repository};
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

#[get("/{slug}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let slug = path.into_inner();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::one(&mut conn, &slug)
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
    tag_json: web::Json<Tag>,
) -> actix_web::Result<impl Responder> {
    let tag = tag_json.into_inner();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::insert(&mut conn, tag)
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match data {
        Ok(exp) => Ok(HttpResponse::Ok().json(exp)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[patch("/{slug}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    tag_json: web::Json<UpdateTag>,
) -> actix_web::Result<impl Responder> {
    let tag = tag_json.into_inner();
    let slug = path.into_inner();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::update(&mut conn, &slug, tag)
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match data {
        Ok(exp) => Ok(HttpResponse::Ok().json(exp)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[delete("/{slug}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let slug = path.into_inner();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::delete(&mut conn, &slug)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    match data {
        Some(exp) => Ok(HttpResponse::Ok().json(exp)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
