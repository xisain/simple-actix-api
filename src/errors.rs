use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    TidakDitemukan(String),
    InputTidakValid(String),
}

// Struct untuk format error response yang konsisten
#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    pesan: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::TidakDitemukan(msg) => write!(f, "{msg}"),
            AppError::InputTidakValid(msg) => write!(f, "{msg}"),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::TidakDitemukan(msg) => HttpResponse::NotFound().json(ErrorResponse {
                status: 404,
                pesan: msg.clone(),
            }),
            AppError::InputTidakValid(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                status: 400,
                pesan: msg.clone(),
            }),
        }
    }
}
