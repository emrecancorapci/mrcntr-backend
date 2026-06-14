use actix_web::{HttpResponse, error::ResponseError};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::Serialize;

use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
    Unauthorized(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: Option<String>,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(err) => HttpResponse::NotFound().json(ErrorResponse {
                error: err.to_string(),
                code: None,
            }),
            AppError::BadRequest(err) => HttpResponse::BadRequest().json(ErrorResponse {
                error: err.to_string(),
                code: None,
            }),
            AppError::Unauthorized(err) => {
                return HttpResponse::Unauthorized().json(ErrorResponse {
                    error: err.to_string(),
                    code: None,
                });
            }
            AppError::Internal(err) => {
                eprintln!("[SERVER ERROR] Internal failure: {}", err);

                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal Server Error".to_string(),
                    code: None,
                });
            }
        }
    }
}

impl Into<HttpResponse> for AppError {
    fn into(self) -> HttpResponse {
        self.error_response()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {msg}"),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {msg}"),
            AppError::Internal(msg) => write!(f, "Internal Server Error: {msg}"),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
        }
    }
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            // Fix: Generic resource not found handler
            DieselError::NotFound => {
                AppError::NotFound("The requested resource was not found.".to_string())
            }

            DieselError::DatabaseError(kind, info) => {
                let target = info
                    .column_name()
                    .or_else(|| info.constraint_name())
                    .unwrap_or("Field");

                match kind {
                    DatabaseErrorKind::UniqueViolation => {
                        AppError::BadRequest(format!("{target} already exists."))
                    }
                    DatabaseErrorKind::ForeignKeyViolation => AppError::BadRequest(format!(
                        "Provided reference in {target} does not exist."
                    )),
                    DatabaseErrorKind::NotNullViolation => {
                        AppError::BadRequest(format!("{target} cannot be empty."))
                    }
                    DatabaseErrorKind::CheckViolation => AppError::BadRequest(format!(
                        "Provided data violates validation rules for {target}."
                    )),
                    DatabaseErrorKind::RestrictViolation => AppError::BadRequest(format!(
                        "Cannot modify resource because {target} depends on it."
                    )),
                    DatabaseErrorKind::ExclusionViolation => AppError::BadRequest(format!(
                        "Values conflict with existing records for {target}."
                    )),
                    DatabaseErrorKind::SerializationFailure => AppError::Internal(
                        "Transaction conflict occurred. Please retry.".to_string(),
                    ),
                    DatabaseErrorKind::ReadOnlyTransaction => {
                        AppError::Internal("Database is in read-only mode.".to_string())
                    }
                    DatabaseErrorKind::UnableToSendCommand => {
                        AppError::Internal("Failed to transmit command to database.".to_string())
                    }
                    DatabaseErrorKind::ClosedConnection => AppError::Internal(
                        "Database connection was unexpectedly closed.".to_string(),
                    ),
                    _ => AppError::Internal("An unexpected database error occurred.".to_string()),
                }
            }
            DieselError::RollbackErrorOnCommit {
                rollback_error,
                commit_error,
            } => AppError::Internal(format!("{rollback_error} - {commit_error}")),
            err => AppError::Internal(err.to_string()),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Internal(value.to_string())
    }
}

impl From<password_hash::Error> for AppError {
    fn from(value: password_hash::Error) -> Self {
        AppError::Internal(value.to_string())
    }
}

impl From<std::fmt::Error> for AppError {
    fn from(value: std::fmt::Error) -> Self {
        AppError::Internal(value.to_string())
    }
}

impl From<Box<std::io::Error>> for AppError {
    fn from(value: Box<std::io::Error>) -> Self {
        AppError::Internal(value.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        AppError::Internal(value.to_string())
    }
}

impl From<actix_web::Error> for AppError {
    fn from(value: actix_web::Error) -> Self {
        AppError::Internal(value.to_string())
    }
}

impl From<actix_web::error::BlockingError> for AppError {
    fn from(value: actix_web::error::BlockingError) -> Self {
        AppError::Internal(value.to_string())
    }
}
