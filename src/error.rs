use std::io::Error;

use serde::Serialize;
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::{StatusCode, header::ContentType};


#[derive(Debug, Serialize)]
pub struct JsonError {
    error: String,
    detail: String,
}


impl JsonError {
    pub fn new(error: &str, detail: &str) -> Self {
        Self { error: error.to_string(), detail: detail.to_string() }
    }
}


impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.detail)
    }
}


impl From<Error> for JsonError {
    fn from(err: Error) -> Self {
        Self {
            error: err.kind().to_string(), 
            detail: err.to_string(),
        }
    }
}


impl ResponseError for JsonError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
