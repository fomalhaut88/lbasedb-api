use serde::Serialize;
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::{StatusCode, header::ContentType};


#[derive(Debug, Serialize)]
pub struct JsonError {
    pub detail: String,
}


impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.detail)
    }
}


impl<E: std::error::Error> From<E> for JsonError {
    fn from(err: E) -> Self {
        Self { detail: err.to_string() }
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
