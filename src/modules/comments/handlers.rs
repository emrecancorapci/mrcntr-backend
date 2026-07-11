use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{Comment, NewComment, UpdateComment, repository};
use crate::{
    DbPool,
    config::error_handler::{AppError, ErrorResponse},
    middlewares::auth::AuthContext,
    modules,
};

const TAG: &str = "Comment";

#[utoipa::path(
    tags = [TAG],
    responses(
        (status = 200, description = "List of comments", body = [Comment]),
        (status = 500, body = ErrorResponse)
    ),
    security(())
)]
#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::many(&mut conn).await.map_err(AppError::from)?;

    return Ok(HttpResponse::Ok().json(data));
}

#[utoipa::path(
    tags = [TAG],
    responses(
        (status = 200, description = "Return single comment by id", body = Comment),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Comment id")
    ),
    security(())
)]
#[get("/{id}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::one(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Comment not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}

#[utoipa::path(
    tags = [TAG],
    request_body = NewComment,
    responses(
        (status = 201, description = "Comment created", body = Comment),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
#[post("")]
pub async fn insert(
    auth: web::ReqData<AuthContext>,
    pool: web::Data<DbPool>,
    body_json: web::Json<NewComment>,
) -> Result<impl Responder, AppError> {
    let mut comment = body_json.into_inner();

    let auth_context = auth.into_inner();
    let uuid = auth_context.user.uuid;
    comment.author_uuid = uuid;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let blogpost = modules::blogposts::repository::one(&mut conn, comment.blogpost_id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Blogpost not found".to_string()))?;

    // TODO: Check if blog post is open to comments
    if !blogpost.is_visible {
        return Err(AppError::unauthorized(
            "You can't comment on this blogpost".to_string(),
        ));
    }

    let data = repository::insert(&mut conn, comment)
        .await
        .map_err(AppError::from)?;

    return Ok(HttpResponse::Created().json(data));
}

#[utoipa::path(
    tags = [TAG],
    request_body = UpdateComment,
    responses(
        (status = 200, description = "Comment updated", body = Comment),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Comment id")
    ),
    security(("token_jwt" = []))
)]
#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<UpdateComment>,
) -> Result<impl Responder, AppError> {
    let comment = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::update(&mut conn, id, comment)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Comment not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}

#[utoipa::path(
    tags = [TAG],
    responses(
        (status = 200, description = "Comment deleted", body = Comment),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Comment id")
    ),
    security(("token_jwt" = []))
)]
#[delete("/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::delete(&mut conn, id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Comment not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}
