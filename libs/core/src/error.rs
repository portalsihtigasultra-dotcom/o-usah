use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    Internal(String),
    Database(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {msg}"),
            AppError::Validation(msg) => write!(f, "Validation error: {msg}"),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {msg}"),
            AppError::Conflict(msg) => write!(f, "Conflict: {msg}"),
            AppError::Internal(msg) => write!(f, "Internal error: {msg}"),
            AppError::Database(msg) => write!(f, "Database error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}