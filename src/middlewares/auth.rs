use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    middleware::Next,
};
use std::{
    io::{Error, ErrorKind},
    pin::Pin,
};
use uuid::Uuid;

use crate::{
    DbPool,
    modules::{auth::helpers, users},
};

#[derive(Clone)]
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

        Ok::<users::UserResponse, Error>(user)
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

pub fn strict_to<B>(
    roles: Vec<&str>,
) -> impl Fn(
    ServiceRequest,
    Next<B>,
) -> Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, actix_web::Error>>>>
+ Clone
where
    B: MessageBody + 'static,
{
    move |req: ServiceRequest, next: Next<B>| {
        let roles: Vec<String> = roles.iter().map(|r| r.to_string()).collect();

        Box::pin(async move {
            let auth_content = req
                .extensions()
                .get::<AuthContent>()
                .cloned()
                .ok_or_else(|| ErrorUnauthorized("Unauthorized: No session found"))?;

            let user_has_role = roles
                .iter()
                .any(|role| Some(role.to_string()) == auth_content.user.role);

            if !user_has_role {
                return Err(ErrorUnauthorized("Forbidden: Insufficient permissions"));
            }

            // 3. Pass request down the pipeline
            next.call(req).await
        })
    }
}
