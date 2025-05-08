use std::io::{Error, ErrorKind};

use serde::Deserialize;
use actix_web::{web, HttpResponse, Result as ActixResult};

use crate::error::JsonError;
use crate::appdata::AppData;


pub type APIResult = ActixResult<HttpResponse, JsonError>;
pub type WebAppData = web::Data<AppData>;


pub fn qs_parse<Q>(qs: &str) -> Result<Q, Error> 
                                where Q: for<'a> Deserialize<'a> {
    let qs = qs.replace("%5B%5D=", "[]=");
    let qs = qs.replace("%5b%5d=", "[]=");
    serde_qs::from_str::<Q>(&qs)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, qs))
}
