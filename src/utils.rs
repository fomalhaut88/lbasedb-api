use actix_web::{web, HttpResponse, Result as ActixResult, Error as ActixError};
use tokio::sync::Mutex;

use crate::appdata::AppData;


pub type APIResult = ActixResult<HttpResponse, ActixError>;
pub type WebAppData = web::Data<Mutex<AppData>>;
