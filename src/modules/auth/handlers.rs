use actix_web::{
    HttpResponse, Responder,
    error::{ErrorBadRequest, ErrorInternalServerError},
    post, web,
};

use super::{LoginRequest, helpers::hash_password};
use crate::{
    DbPool,
    modules::{
        auth::{
            AuthResponse,
            helpers::{generate_jwt, verify_password},
        },
        users::{NewUser, repository},
    },
};

#[post("/register")]
pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> actix_web::Result<impl Responder> {
    let login_request = body.into_inner();

    let hashed_password = hash_password(&login_request.password).map_err(ErrorInternalServerError);

    let new_user = NewUser {
        email: login_request.email,
        password_hash: hashed_password?,
    };

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::insert(&mut conn, new_user)
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match data {
        Ok(user) => {
            let token = generate_jwt(user.uuid.to_string()).map_err(ErrorInternalServerError)?;
            let payload = AuthResponse { token };

            return Ok(HttpResponse::Ok().json(payload));
        }
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> actix_web::Result<impl Responder> {
    let login_request = body.into_inner();

    let data = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        repository::one_by_email(&mut conn, &login_request.email)
    })
    .await
    .map_err(ErrorInternalServerError)?;

    match data {
        Ok(Some(user)) => {
            verify_password(&login_request.password, &user.password_hash).map_err(|err| {
                dbg!(err.to_string());

                ErrorBadRequest(err)
            })?;

            let token = generate_jwt(user.uuid.to_string()).map_err(ErrorInternalServerError)?;

            return Ok(HttpResponse::Ok().json(token));
        }
        Ok(None) => Ok(HttpResponse::BadRequest().finish()),
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}
