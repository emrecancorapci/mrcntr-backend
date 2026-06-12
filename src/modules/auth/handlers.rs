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
        email: login_request.email,
        password_hash: hashed_password,
    };

    let user = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::insert(&mut conn, new_user).map_err(AppError::from)
    })
    .await??; 

    let token = generate_jwt(user.uuid.to_string())?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let login_request = body.into_inner();

    let data = web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| AppError::Internal(err.to_string()))?;

        repository::one_by_email(&mut conn, &login_request.email).map_err(AppError::from)
    })
    .await??;

    let user = data.ok_or_else(|| AppError::BadRequest("Credentials are wrong.".to_string()))?;

    verify_password(&login_request.password, &user.password_hash)?;

    let token = generate_jwt(user.uuid.to_string())?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}