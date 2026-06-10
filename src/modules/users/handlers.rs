use actix_web::{
    HttpResponse, Responder, delete, error::ErrorInternalServerError, get, patch, post, web,
};

use super::{NewUser, NewUserBody, UpdateUser, repository};
use crate::{
    DbPool,
    modules::{auth::helpers::hash_password, users::UpdateUserBody},
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

#[get("/{uuid}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let uuid_str = path.into_inner();
    let uuid = uuid_str
        .parse::<uuid::Uuid>()
        .map_err(|_| ErrorInternalServerError("Invalid UUID format"))?;

    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::one(&mut conn, uuid)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    match result {
        Some(data) => Ok(HttpResponse::Ok().json(data)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body: web::Json<NewUserBody>,
) -> actix_web::Result<impl Responder> {
    let body = body.into_inner();
    let hash = hash_password(&body.password).map_err(ErrorInternalServerError)?;
    let new_user = NewUser {
        email: body.email,
        password_hash: hash,
    };

    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::insert(&mut conn, new_user)
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match result {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[patch("/{uuid}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateUserBody>,
) -> actix_web::Result<impl Responder> {
    let body = body.into_inner();
    let uuid_str = path.into_inner();
    let uuid = uuid_str
        .parse::<uuid::Uuid>()
        .map_err(|_| ErrorInternalServerError("Invalid UUID format"))?;

    let hash = body
        .password
        .map(|p| hash_password(&p))
        .transpose()
        .map_err(ErrorInternalServerError)?;
    let update_user = UpdateUser {
        email: body.email,
        password_hash: hash,
    };

    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::update(&mut conn, uuid, update_user)
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match result {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[delete("/{uuid}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let uuid_str = path.into_inner();
    let uuid = uuid_str
        .parse::<uuid::Uuid>()
        .map_err(|_| ErrorInternalServerError("Invalid UUID format"))?;

    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::delete(&mut conn, uuid)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    match result {
        Some(data) => Ok(HttpResponse::Ok().json(data)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
