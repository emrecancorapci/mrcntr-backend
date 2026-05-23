use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, get, web};

use super::repository;
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
