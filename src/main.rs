mod utils;
mod config;
mod appdata;
mod resource;

use actix_web::{get, web, App, HttpResponse, HttpServer};

use crate::config::Config;
use crate::appdata::AppData;
use crate::resource::*;


#[get("/version")]
async fn version_view() -> HttpResponse {
    let version = env!("CARGO_PKG_VERSION");
    HttpResponse::Ok().body(version)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    let instance = AppData::new(&config.data_path).await?;
    let appdata = web::Data::new(instance);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(appdata.clone())
            .service(version_view)
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
