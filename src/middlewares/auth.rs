use crate::{
    REDIS_USER_DATA, REDIS_USER_TOKEN, RedisPool,
    modules::{
        auth::helpers,
        users::{self, UserResponse},
    },
};

use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    middleware::Next,
    web,
};
use redis::AsyncTypedCommands;
use std::pin::Pin;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthContext {
    pub user: users::UserResponse,
}

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let redis_pool = req.app_data::<web::Data<RedisPool>>().ok_or_else(|| {
        eprintln!("[SERVER ERROR] Redis Pool missing from app_data");
        ErrorInternalServerError("Internal Server Error")
    })?;

    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ErrorUnauthorized("Authorization header missing"))?
        .to_str()
        .map_err(|err| {
            eprintln!("[SERVER ERROR] Non-utf8 character in Auth header: {}", err);
            ErrorUnauthorized("Malformed authorization header")
        })?;

    let Some((bearer, token)) = auth_header.split_once(' ') else {
        return Err(ErrorUnauthorized("Malformed authorization header"));
    };

    if bearer != "Bearer" {
        return Err(ErrorUnauthorized("Malformed authorization header"));
    }

    let claim = helpers::decode_jwt(token).map_err(|err| {
        eprintln!("[SERVER ERROR] JWT decoding failed: {}", err);
        ErrorUnauthorized("Invalid or expired token")
    })?;

    let uuid = Uuid::parse_str(&claim.claims.uuid).map_err(|err| {
        eprintln!(
            "[SERVER ERROR] Failed to parse UUID from token claims: {}",
            err
        );
        ErrorUnauthorized("Invalid token payload")
    })?;

    let mut redis = redis_pool.get().await.map_err(|err| {
        eprintln!("[SERVER ERROR] Redis connection failed: {}", err);
        ErrorInternalServerError("Internal Server Error")
    })?;

    let token_key = format!("{}{}", REDIS_USER_TOKEN, uuid);
    let data_key = format!("{}{}", REDIS_USER_DATA, uuid);
    let redis_data = redis.mget(&[token_key, data_key]).await.map_err(|err| {
        eprintln!("[SERVER ERROR] Redis GET failed: {}", err);
        ErrorInternalServerError("Internal Server Error")
    })?;

    let cached_token = redis_data.get(0).ok_or_else(|| {
        eprintln!("[SERVER ERROR] Redis GET failed: Redis response is missing(0)");
        ErrorInternalServerError("Internal Server Error")
    })?;

    let cached_user_data = redis_data.get(1).ok_or_else(|| {
        eprintln!("[SERVER ERROR] Redis GET failed: Redis response is missing(1)");
        ErrorInternalServerError("Internal Server Error")
    })?;

    if let Some(t) = cached_token {
        if t != token {
            return Err(ErrorUnauthorized("Session has expired or been invalidated"));
        }
    } else {
        return next.call(req).await;
    }

    if let Some(user_data_str) = cached_user_data {
        let user: UserResponse = serde_json::from_str(user_data_str).map_err(|err| {
            eprintln!("[SERVER ERROR] Json deserialization error: {}", err);
            ErrorInternalServerError("Internal Server Error")
        })?;

        let auth_content = AuthContext { user };
        req.extensions_mut().insert(auth_content);
    } else {
        return next.call(req).await;
    }

    next.call(req).await
}

pub fn strict_to<B>(
    roles: &[&str],
) -> impl Fn(
    ServiceRequest,
    Next<B>,
) -> Pin<Box<dyn Future<Output = Result<ServiceResponse<B>, actix_web::Error>>>>
+ Clone
where
    B: MessageBody + 'static,
{
    let roles: Vec<String> = roles.iter().map(|r| r.to_string()).collect();

    move |req: ServiceRequest, next: Next<B>| {
        let roles = roles.clone();

        Box::pin(async move {
            let auth_content = req
                .extensions()
                .get::<AuthContext>()
                .cloned()
                .ok_or_else(|| ErrorUnauthorized("Unauthorized: No session found"))?;

            let user_has_role = roles
                .iter()
                .any(|role| Some(role.to_string()) == auth_content.user.role);

            if !user_has_role {
                return Err(ErrorUnauthorized("Forbidden: Insufficient permissions"));
            }

            next.call(req).await
        })
    }
}
