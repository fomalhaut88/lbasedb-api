use actix_web::{web, HttpResponse, Result as ActixResult};

use crate::error::JsonError;
use crate::appdata::AppData;


pub type APIResult = ActixResult<HttpResponse, JsonError>;
pub type WebAppData = web::Data<AppData>;
