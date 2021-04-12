use crate::result::Error;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(error) => {
                HttpResponse::BadRequest().json(error)
            }
            Error::NotFound(message) => {
                HttpResponse::NotFound().json(message)
            }
            Error::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json(errors.to_vec())
            }
            Error::Unauthorized(error) => {
                HttpResponse::Unauthorized().json(error)
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

/// Utility to make transforming a string reference into an ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// User-friendly error messages
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Utility to make transforming a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}
