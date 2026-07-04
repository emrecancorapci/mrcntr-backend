use actix_web::{HttpResponse, Responder, post, web};

use super::{
    AuthResponse, LoginRequest,
    helpers::{generate_jwt, hash_password, verify_password},
};
use crate::{
    DbPool,
    config::error_handler::AppError,
    modules::users::{NewUser, repository},
};

#[post("/register")]
pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
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

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let user = repository::insert(&mut conn, new_user)
        .await
        .map_err(AppError::from)?;

    let token = generate_jwt(user.uuid.to_string())?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let login_request = body.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let user = repository::one_by_email(&mut conn, &login_request.email)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::bad_request("Credentials are wrong.".to_string()))?;

    verify_password(&login_request.password, &user.password_hash)?;

    let token = generate_jwt(user.uuid.to_string())?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}
