use super::{NewUser, NewUserBody, UpdateUser, UpdateUserBody, UserResponse, repository};
use crate::{AppError, DbPool, config::error_handler::ErrorResponse, modules::auth::helpers::hash_password};

use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use validator::Validate;

#[utoipa::path(
    tags = ["User"],
    responses(
        (status = 200, description = "List of users", body = [UserResponse]),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let users = repository::many(&mut conn).await.map_err(AppError::from)?;
    let data: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["User"],
    responses(
        (status = 200, description = "User detail", body = UserResponse),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("uuid" = String, Path, description = "User UUID")
    ),
    security(("token_jwt" = []))
)]
#[get("/{uuid}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let uuid_str = path.into_inner();
    let uuid = uuid_str
        .parse::<uuid::Uuid>()
        .map_err(|_| AppError::bad_request("Invalid UUID format".to_string()))?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let user = repository::one(&mut conn, uuid)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

    let data: UserResponse = user.into();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["User"],
    request_body = NewUserBody,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body: web::Json<NewUserBody>,
) -> Result<impl Responder, AppError> {
    let body = body.into_inner();

    body.validate()?;

    let hash = hash_password(&body.password).map_err(AppError::from)?;
    let new_user = NewUser::from_body(body, &hash);

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: UserResponse = repository::insert(&mut conn, new_user)
        .await
        .map_err(AppError::from)?
        .into();

    Ok(HttpResponse::Created().json(data))
}

#[utoipa::path(
    tags = ["User"],
    request_body = UpdateUserBody,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("uuid" = String, Path, description = "User UUID")
    ),
    security(("token_jwt" = []))
)]
#[patch("/{uuid}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateUserBody>,
) -> Result<impl Responder, AppError> {
    let body = body.into_inner();

    body.validate()?;

    let uuid_str = path.into_inner();
    let uuid = uuid_str
        .parse::<uuid::Uuid>()
        .map_err(|_| AppError::bad_request("Invalid UUID format".to_string()))?;

    let hash = body
        .password
        .clone()
        .map(|p| hash_password(&p))
        .transpose()
        .map_err(AppError::from)?;
    let update_user = UpdateUser::from_body(body, hash);

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let user = repository::update(&mut conn, uuid, update_user)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

    let data: UserResponse = user.into();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["User"],
    responses(
        (status = 200, description = "User deleted", body = UserResponse),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("uuid" = String, Path, description = "User UUID")
    ),
    security(("token_jwt" = []))
)]
#[delete("/{uuid}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let uuid_str = path.into_inner();
    let uuid = uuid_str
        .parse::<uuid::Uuid>()
        .map_err(|_| AppError::bad_request("Invalid UUID format".to_string()))?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: UserResponse = repository::delete(&mut conn, uuid)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("User not found".to_string()))?
        .into();

    Ok(HttpResponse::Ok().json(data))
}
