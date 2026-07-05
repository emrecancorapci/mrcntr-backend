use super::{
    AuthResponse, LoginRequest,
    helpers::{generate_jwt, hash_password, verify_password},
};
use crate::{
    DbPool, REDIS_USER_DATA, REDIS_USER_TOKEN, RedisPool,
    config::error_handler::AppError,
    modules::users::{NewUser, UserResponse, repository},
};

use actix_web::{HttpResponse, Responder, post, web};

#[post("/register")]
pub async fn register(
    pool: web::Data<DbPool>,
    redis_pool: web::Data<RedisPool>,
    body: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let mut redis = redis_pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let login_request = body.into_inner();

    let hashed_password = hash_password(&login_request.password)?;

    let new_user = NewUser {
        first_name: None,
        last_name: None,
        summary: None,
        image_url: None,
        email: login_request.email,
        password_hash: hashed_password,
        role_id: 3,
    };

    let user_response = repository::insert(&mut conn, new_user)
        .await
        .map_err(AppError::from)?;

    let token = generate_jwt(&user_response.uuid.to_string())?;
    let uuid = user_response.uuid.to_string();

    let user_response_json = serde_json::to_string(&user_response).map_err(AppError::from)?;

    redis::pipe()
        .cmd("SET")
        .atomic()
        .arg(&[format!("{}{}", REDIS_USER_TOKEN, &uuid), token.to_string()])
        .ignore()
        .cmd("SET")
        .arg(&[format!("{}{}", REDIS_USER_DATA, &uuid), user_response_json])
        .ignore()
        .query_async::<()>(&mut *redis)
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    redis_pool: web::Data<RedisPool>,
    body: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let mut redis = redis_pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let login_request = body.into_inner();

    let user = repository::one_by_email(&mut conn, &login_request.email)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::bad_request("Credentials are wrong.".to_string()))?;

    verify_password(&login_request.password, &user.password_hash)?;

    let uuid = user.uuid.to_string();
    let token = generate_jwt(&uuid)?;

    let user_response: UserResponse = user.into();
    let user_response_json = serde_json::to_string(&user_response).map_err(AppError::from)?;

    redis::pipe()
        .cmd("SET")
        .atomic()
        .arg(&[format!("{}{}", REDIS_USER_TOKEN, &uuid), token.to_string()])
        .ignore()
        .cmd("SET")
        .arg(&[format!("{}{}", REDIS_USER_DATA, &uuid), user_response_json])
        .ignore()
        .query_async::<()>(&mut *redis)
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}
