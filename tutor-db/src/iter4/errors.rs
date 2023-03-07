use std::fmt::Pointer;

use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SqlxError;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DbError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorRsponse {
    error_message: String,
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DbError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            EzyTutorError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl std::fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<SqlxError> for EzyTutorError {
    fn from(error: SqlxError) -> Self {
        EzyTutorError::DbError(error.to_string())
    }
}

impl From<actix_web::error::Error> for EzyTutorError {
    fn from(error: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(error.to_string())
    }
}

impl error::ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DbError(_msg) | EzyTutorError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorRsponse {
            error_message: self.error_response(),
        })
    }
}
