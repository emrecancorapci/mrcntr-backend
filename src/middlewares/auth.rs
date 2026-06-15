use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    middleware::Next,
};
use std::io::{Error, ErrorKind};
use uuid::Uuid;

use crate::{
    DbPool,
    modules::{auth::helpers, users},
};

struct AuthContent {
    user: users::UserResponse,
}

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ErrorUnauthorized("No token found"))?
        .to_str()
        .map_err(|err| {
            eprintln!("[SERVER ERROR] Internal failure: {}", err);
            ErrorUnauthorized("Malformed authorization header")
        })?;

    let token = if let Some((bearer, token)) = auth_header.split_once(' ')
        && bearer == "Bearer"
    {
        Ok(token)
    } else {
        Err(ErrorUnauthorized("Malformed authorization header"))
    }?;

    let claim = helpers::decode_jwt(token.to_string()).map_err(|err| {
        eprintln!("[SERVER ERROR] Internal failure: {}", err);
        ErrorUnauthorized("Malformed token detected")
    })?;
    let uuid = Uuid::parse_str(&claim.claims.uuid).map_err(|err| {
        eprintln!("[SERVER ERROR] Internal failure: {}", err);
        ErrorUnauthorized("Malformed token detected")
    })?;

    let pool = req
        .app_data::<actix_web::web::Data<DbPool>>()
        .cloned()
        .ok_or_else(|| {
            eprintln!("Database connection failed");
            ErrorInternalServerError("Internal Server Error")
        })?;

    let user_response = actix_web::web::block(move || {
        let mut conn = pool
            .get()
            .map_err(|err| Error::new(ErrorKind::ConnectionRefused, err))?;

        let user = users::repository::one(&mut conn, uuid)
            .map_err(|err| Error::new(ErrorKind::ConnectionRefused, err))?
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "User not found"))?;

        return Ok::<users::UserResponse, Error>(user);
    })
    .await
    .map_err(|_| ErrorInternalServerError("Internal Server Error"))?
    .map_err(|err| match err.kind() {
        ErrorKind::NotFound => ErrorUnauthorized("Malformed token detected"),
        _ => {
            eprintln!("[SERVER ERROR] Internal failure: {}", err);
            ErrorInternalServerError("Internal Server Error")
        }
    })?;

    let auth_content = AuthContent {
        user: user_response,
    };

    req.extensions_mut().insert(auth_content);

    next.call(req).await
}
