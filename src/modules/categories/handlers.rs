use super::{CategoryResponse, NewCategory, UpdateCategory, UpdateCategoryRequest, repository};
use crate::{DbPool, config::error_handler::{AppError, ErrorResponse}};

use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use chrono::Utc;
use validator::Validate;

#[utoipa::path(
    tags = ["Category"],
    responses(
        (status = 200, description = "List of categories", body = [CategoryResponse]),
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

    let data: Vec<CategoryResponse> = repository::many(&mut conn)
        .await
        .map_err(AppError::from)?
        .into_iter()
        .map(|c| c.into())
        .collect();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Category"],
    responses(
        (status = 200, description = "Category detail", body = CategoryResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("slug" = String, Path, description = "Category slug")
    ),
    security(())
)]
#[get("/{slug}")]
pub async fn one(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let slug = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: CategoryResponse = repository::one(&mut conn, &slug)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Category not found".to_string()))?
        .into();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Category"],
    request_body = NewCategory,
    responses(
        (status = 201, description = "Category created", body = CategoryResponse),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    security(("token_jwt" = []))
)]
#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body: web::Json<NewCategory>,
) -> Result<impl Responder, AppError> {
    let category = body.into_inner();

    category.validate()?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: CategoryResponse = repository::insert(&mut conn, category)
        .await
        .map_err(AppError::from)?
        .into();

    Ok(HttpResponse::Created().json(data))
}

#[utoipa::path(
    tags = ["Category"],
    request_body = UpdateCategoryRequest,
    responses(
        (status = 200, description = "Category updated", body = CategoryResponse),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("slug" = String, Path, description = "Category slug")
    ),
    security(("token_jwt" = []))
)]
#[patch("/{slug}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateCategoryRequest>,
) -> Result<impl Responder, AppError> {
    let category_req = body.into_inner();
    let slug = path.into_inner();

    category_req.validate()?;

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let category = UpdateCategory {
        slug: category_req.slug,
        title: category_req.title,
        updated_at: Utc::now(),
    };

    let data: CategoryResponse = repository::update(&mut conn, &slug, category)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Category not found".to_string()))?
        .into();

    Ok(HttpResponse::Ok().json(data))
}

#[utoipa::path(
    tags = ["Category"],
    responses(
        (status = 200, description = "Category deleted", body = CategoryResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("slug" = String, Path, description = "Category slug")
    ),
    security(("token_jwt" = []))
)]
#[delete("/{slug}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let slug = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data: CategoryResponse = repository::delete(&mut conn, &slug)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("Category not found".to_string()))?
        .into();

    Ok(HttpResponse::Ok().json(data))
}
