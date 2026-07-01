use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{NewUser, NewUserBody, UpdateUser, repository};
use crate::{
    DbPool,
    config::error_handler::AppError,
    modules::{auth::helpers::hash_password, users::UpdateUserBody},
};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::many(&mut conn).await.map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(data))
}

#[get("/{uuid}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let uuid_str = path.into_inner();
    let uuid = uuid_str
        .parse::<uuid::Uuid>()
        .map_err(|_| AppError::BadRequest("Invalid UUID format".to_string()))?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::one(&mut conn, uuid)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body: web::Json<NewUserBody>,
) -> Result<impl Responder, AppError> {
    let body = body.into_inner();
    let hash = hash_password(&body.password).map_err(AppError::from)?;
    let new_user = NewUser {
        email: body.email,
        password_hash: hash,
        role_id: 3,
    };

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::insert(&mut conn, new_user)
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Created().json(data))
}

#[patch("/{uuid}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateUserBody>,
) -> Result<impl Responder, AppError> {
    let body = body.into_inner();
    let uuid_str = path.into_inner();
    let uuid = uuid_str
        .parse::<uuid::Uuid>()
        .map_err(|_| AppError::BadRequest("Invalid UUID format".to_string()))?;

    let hash = body
        .password
        .map(|p| hash_password(&p))
        .transpose()
        .map_err(AppError::from)?;
    let update_user = UpdateUser {
        email: body.email,
        password_hash: hash,
    };

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::update(&mut conn, uuid, update_user)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}

#[delete("/{uuid}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let uuid_str = path.into_inner();
    let uuid = uuid_str
        .parse::<uuid::Uuid>()
        .map_err(|_| AppError::BadRequest("Invalid UUID format".to_string()))?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::Internal(err.to_string()))?;

    let data = repository::delete(&mut conn, uuid)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(data))
}
