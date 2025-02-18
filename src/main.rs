mod utils;
mod config;
mod appdata;
mod resource;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use tokio::sync::Mutex;

use crate::config::Config;
use crate::appdata::AppData;
use crate::resource::*;


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home\n")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    let instance = AppData::new().await?;
    let appdata = web::Data::new(Mutex::new(instance));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(appdata.clone())
            .service(home)
            .service(load_resource_data())
            .service(load_resource_feed())
            .service(load_resource_col())
            .service(load_resource_size())
            .service(load_resource_raw())
    })
        .workers(config.workers)
        .bind((config.host, config.port))?;

    server.run().await
}
