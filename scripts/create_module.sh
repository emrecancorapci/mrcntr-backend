#!/usr/bin/env bash
set -euo pipefail

if [ -z "${1:-}" ]; then
	echo "Error: Please provide a module name. Example: $0 party-crashes"
	exit 1
fi

if ((BASH_VERSINFO[0] < 4)); then
	echo "Error: This script requires Bash 4.0 or higher."
	echo "You are currently running Bash ${BASH_VERSION}."
	echo "Please upgrade Bash (e.g., 'brew install bash' on macOS) or update your shebang."
	exit 1
fi

RAW_INPUT="$1"

SNAKE_INPUT="${RAW_INPUT//-/_}" # Replace hyphens with underscores for snake_case
MODULE_NAME="${SNAKE_INPUT,,}"  # Force entire string lowercase

# Capitalize/PascalCase conversions
pascal_name=""
IFS='_ ' read -r -a parts <<<"${MODULE_NAME}"
for part in "${parts[@]}"; do
	pascal_name="${pascal_name}${part^}"
done

# Strip off plural endings (Exams -> Exam, Activities -> Activity)
if [[ "$pascal_name" == *ies ]]; then
	MODEL_NAME="${pascal_name%ies}y"
else
	MODEL_NAME="${pascal_name%s}"
fi
VARIABLE_NAME="${MODEL_NAME,,}"

MOD_FOLDER="src/modules/$MODULE_NAME"

create_module() {
	cat <<EOF
mod handlers;
mod models;
pub mod repository;

pub use handlers::*;
pub use models::*;
EOF
}

create_handler() {
	cat <<EOF
use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};

use super::{New$MODEL_NAME, Update$MODEL_NAME, repository};
use crate::{DbPool, config::error_handler::AppError};

#[get("")]
pub async fn many(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::many(&mut conn).await
        .map_err(AppError::from)?;
        
    return Ok(HttpResponse::Ok().json(data));
}

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

    let data = repository:one(&mut conn, id)
		.await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("$MODEL_NAME not found".to_string()))?

    return Ok(HttpResponse::Ok().json(data));
}

#[post("")]
pub async fn insert(
    pool: web::Data<DbPool>,
    body_json: web::Json<New$MODEL_NAME>,
) -> Result<impl Responder, AppError> {
    let $VARIABLE_NAME = body_json.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::insert(&mut conn, $VARIABLE_NAME)
		.await
        .map_err(AppError::from)?;

    return Ok(HttpResponse::Created().json(data));
}

#[patch("/{id}")]
pub async fn update(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body_json: web::Json<Update$MODEL_NAME>
) -> Result<impl Responder, AppError> {
    let $VARIABLE_NAME = body_json.into_inner();
    let id = path.into_inner();

    let mut conn = pool
        .get()
        .await
        .map_err(|err| AppError::internal(err.to_string()))?;

    let data = repository::update(&mutconn, id, $VARIABLE_NAME)
		.await
        .map_err(AppError::from)?
		.ok_or_else(|| AppError::not_found("$MODEL_NAME not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}
            
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

    let data = repository:delete(&mut conn, id)
		.await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::not_found("$MODEL_NAME not found".to_string()))?;

    return Ok(HttpResponse::Ok().json(data));
}
EOF
}

create_models() {
	cat <<EOF
use crate::config::schema;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Selectable, Identifiable, Validate, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::$MODULE_NAME)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct $MODEL_NAME {
	pub id: i32,

}

#[derive(Insertable, Debug, Validate, Clone, Deserialize)]
#[diesel(table_name = schema::$MODULE_NAME)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct New$MODEL_NAME {

}

#[derive(AsChangeset, Validate, Deserialize)]
#[diesel(table_name = schema::$MODULE_NAME)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Update$MODEL_NAME {

}
EOF
}

create_repository() {
	cat <<EOF
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper, result::Error};
use diesel_async::RunQueryDsl;

use super::{New$MODEL_NAME, $MODEL_NAME, Update$MODEL_NAME};
use crate::{PooledConn, schema::$MODULE_NAME};

pub async fn one(conn: &mut PooledConn, id: i32) -> Result<Option<$MODEL_NAME>, Error> {
    $MODULE_NAME::table
        .find(id)
        .first::<$MODEL_NAME>(conn)
        .await
		.optional()
}

pub async fn many(conn: &mut PooledConn) -> Result<Vec<$MODEL_NAME>, Error> {
    $MODULE_NAME::table
        .order_by($MODULE_NAME::id.desc())
        .load::<$MODEL_NAME>(conn)
		.await
}

pub async fn insert(conn: &mut PooledConn, $VARIABLE_NAME: New$MODEL_NAME) -> Result<$MODEL_NAME, Error> {
    diesel::insert_into($MODULE_NAME::table)
        .values(&$VARIABLE_NAME)
        .returning($MODEL_NAME::as_returning())
        .get_result(conn)
		.await
}

pub async fn update(
    conn: &mut PooledConn,
    id: i32,
    $VARIABLE_NAME: Update$MODEL_NAME,
) -> Result<Option<$MODEL_NAME>, Error> {
    diesel::update($MODULE_NAME::dsl::$MODULE_NAME.find(id))
        .set($VARIABLE_NAME)
        .returning($MODEL_NAME::as_returning())
        .get_result(conn)
        .await
		.optional()
}

pub async fn delete(conn: &mut PooledConn, id: i32) -> Result<Option<$MODEL_NAME>, Error> {
    diesel::delete($MODULE_NAME::dsl::$MODULE_NAME.filter($MODULE_NAME::id.eq(id)))
        .returning($MODEL_NAME::as_returning())
        .get_result(conn)
        .await
		.optional()
}
EOF
}

## Main

mkdir -p $MOD_FOLDER
create_module >"$MOD_FOLDER.rs"

create_handler >"$MOD_FOLDER/handlers.rs"
create_models >"$MOD_FOLDER/models.rs"
create_repository >"$MOD_FOLDER/repository.rs"

echo "pub mod $MODULE_NAME;" >>src/modules.rs

## Final Output Response
echo "$MODEL_NAME ($MODULE_NAME) module created successfully!"
