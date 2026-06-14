use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
};

use crate::modules::auth::helpers::decode_jwt;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let (req_part, _res) = req.parts();

    let auth_header = req_part
        .headers()
        .get("Authorization")
        .ok_or_else(|| ErrorUnauthorized("No token found"))?;

    let auth_str = auth_header.to_str().map_err(ErrorUnauthorized)?;

    let token = if let Some((bearer, token)) = auth_str.split_once(' ')
        && bearer == "Bearer"
    {
        Ok(token)
    } else {
        Err(ErrorUnauthorized("Malformed Authorization header"))
    }?;

    let claim = decode_jwt(token.to_string())
        .map_err(|_| ErrorUnauthorized("Malformed token detected"))?;

    req.extensions_mut().insert(claim);

    next.call(req).await
}
