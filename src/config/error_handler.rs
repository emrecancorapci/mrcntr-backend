use actix_web::{HttpResponse, error::ResponseError};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::Serialize;

use std::{collections::HashSet, fmt};

#[derive(Debug)]
pub enum AppError {
    NotFound(HashSet<String>),
    BadRequest(HashSet<String>),
    Internal(HashSet<String>),
    Unauthorized(HashSet<String>),
}

#[derive(Serialize)]
struct ErrorResponse {
    errors: HashSet<String>,
    codes: HashSet<String>,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(err) => HttpResponse::NotFound().json(ErrorResponse {
                errors: err.clone(),
                codes: HashSet::new(),
            }),
            AppError::BadRequest(err) => HttpResponse::BadRequest().json(ErrorResponse {
                errors: err.clone(),
                codes: HashSet::new(),
            }),
            AppError::Unauthorized(err) => HttpResponse::Unauthorized().json(ErrorResponse {
                errors: err.clone(),
                codes: HashSet::new(),
            }),
            AppError::Internal(err) => {
                eprintln!("[SERVER ERROR] Internal failure: {}", join_errors(err));

                HttpResponse::InternalServerError().json(ErrorResponse {
                    errors: HashSet::from(["Internal Server Error".to_string()]),
                    codes: HashSet::new(),
                })
            }
        }
    }
}

impl AppError {
    pub fn internal(msg: String) -> AppError {
        AppError::Internal(HashSet::from([msg]))
    }

    pub fn not_found(msg: String) -> AppError {
        AppError::NotFound(HashSet::from([msg]))
    }

    pub fn bad_request(msg: String) -> AppError {
        AppError::BadRequest(HashSet::from([msg]))
    }

    pub fn unauthorized(msg: String) -> AppError {
        AppError::Unauthorized(HashSet::from([msg]))
    }

    pub fn into_messages(self) -> HashSet<String> {
        match self {
            AppError::NotFound(set) => set,
            AppError::BadRequest(set) => set,
            AppError::Internal(set) => set,
            AppError::Unauthorized(set) => set,
        }
    }
}

impl From<AppError> for HttpResponse {
    fn from(val: AppError) -> Self {
        val.error_response()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", join_errors(msg)),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", join_errors(msg)),
            AppError::Internal(msg) => write!(f, "Internal Server Error: {}", join_errors(msg)),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", join_errors(msg)),
        }
    }
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            // Fix: Generic resource not found handler
            DieselError::NotFound => {
                AppError::not_found("The requested resource was not found.".to_string())
            }

            DieselError::DatabaseError(kind, info) => {
                let target = info
                    .column_name()
                    .or_else(|| info.constraint_name())
                    .unwrap_or("Field");

                match kind {
                    DatabaseErrorKind::UniqueViolation => {
                        AppError::bad_request(format!("{target} already exists."))
                    }
                    DatabaseErrorKind::ForeignKeyViolation => AppError::bad_request(format!(
                        "Provided reference in {target} does not exist."
                    )),
                    DatabaseErrorKind::NotNullViolation => {
                        AppError::bad_request(format!("{target} cannot be empty."))
                    }
                    DatabaseErrorKind::CheckViolation => AppError::bad_request(format!(
                        "Provided data violates validation rules for {target}."
                    )),
                    DatabaseErrorKind::RestrictViolation => AppError::bad_request(format!(
                        "Cannot modify resource because {target} depends on it."
                    )),
                    DatabaseErrorKind::ExclusionViolation => AppError::bad_request(format!(
                        "Values conflict with existing records for {target}."
                    )),
                    DatabaseErrorKind::SerializationFailure => AppError::internal(
                        "Transaction conflict occurred. Please retry.".to_string(),
                    ),
                    DatabaseErrorKind::ReadOnlyTransaction => {
                        AppError::internal("Database is in read-only mode.".to_string())
                    }
                    DatabaseErrorKind::UnableToSendCommand => {
                        AppError::internal("Failed to transmit command to database.".to_string())
                    }
                    DatabaseErrorKind::ClosedConnection => AppError::internal(
                        "Database connection was unexpectedly closed.".to_string(),
                    ),
                    _ => AppError::internal("An unexpected database error occurred.".to_string()),
                }
            }
            DieselError::RollbackErrorOnCommit {
                rollback_error,
                commit_error,
            } => AppError::internal(format!("{rollback_error} - {commit_error}")),
            err => AppError::internal(err.to_string()),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::internal(value.to_string())
    }
}

impl From<password_hash::Error> for AppError {
    fn from(value: password_hash::Error) -> Self {
        AppError::internal(value.to_string())
    }
}

impl From<std::fmt::Error> for AppError {
    fn from(value: std::fmt::Error) -> Self {
        AppError::internal(value.to_string())
    }
}

impl From<Box<std::io::Error>> for AppError {
    fn from(value: Box<std::io::Error>) -> Self {
        AppError::internal(value.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        AppError::internal(value.to_string())
    }
}

impl From<actix_web::Error> for AppError {
    fn from(value: actix_web::Error) -> Self {
        AppError::internal(value.to_string())
    }
}

impl From<actix_web::error::BlockingError> for AppError {
    fn from(value: actix_web::error::BlockingError) -> Self {
        AppError::internal(value.to_string())
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(value: validator::ValidationErrors) -> Self {
        let errs = value.errors();
        let mut final_errors = HashSet::new();

        for (field, kind) in errs {
            match kind {
                validator::ValidationErrorsKind::Struct(nested_errs) => {
                    let app_err: AppError = (**nested_errs).clone().into();
                    for msg in app_err.into_messages() {
                        final_errors.insert(format!("[{}]: {}", field, msg));
                    }
                }
                validator::ValidationErrorsKind::List(btree_map) => {
                    for (index, nested_err) in btree_map {
                        let app_err: AppError = (**nested_err).clone().into();
                        for msg in app_err.into_messages() {
                            final_errors.insert(format!("[{} at index {}]: {}", field, index, msg));
                        }
                    }
                }
                validator::ValidationErrorsKind::Field(errors) => {
                    for e in errors {
                        let msg = e
                            .message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| "is invalid".to_string());
                        final_errors.insert(format!("{}: {}", field, msg));
                    }
                }
            }
        }

        AppError::BadRequest(final_errors)
    }
}

fn join_errors(set: &HashSet<String>) -> String {
    set.iter().cloned().collect::<Vec<_>>().join(", ")
}
