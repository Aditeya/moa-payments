use actix_web::{body::BoxBody, HttpResponse, ResponseError};
use thiserror::Error;

use crate::db;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AppError {
    #[error("Database Failure: {0}")]
    DatabaseFailure(#[from] db::Error),
    #[error("pwhash Hash Failure: {0}")]
    CryptographyFailure(#[from] pwhash::error::Error),
    #[error("BadRequest: {0}")]
    BadRequest(String),
    #[error("Resource Gone: {0}")]
    Gone(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::Gone(_) => actix_web::http::StatusCode::GONE,
            Self::BadRequest(_) => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let msg = match self {
            Self::DatabaseFailure(_) => "Database Failure",
            Self::CryptographyFailure(_) => "Crypto Failure",
            Self::BadRequest(a) => a.as_str(),
            Self::Gone(a) => a.as_str(),
        };

        let code = self.status_code();
        HttpResponse::build(code).body(format!("{code}: {msg}"))
    }
}
