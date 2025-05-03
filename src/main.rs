mod utils;
mod error;
mod config;
mod appdata;
mod resource;

use serde::Serialize;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use actix_web::http::header;
use actix_cors::Cors;

use crate::utils::APIResult;
use crate::config::Config;
use crate::appdata::AppData;
use crate::resource::*;


#[derive(Serialize)]
struct VersionInfo {
    version: String,
}


#[get("/version")]
async fn version_view() -> APIResult {
    let version = env!("CARGO_PKG_VERSION").to_string();
    Ok(HttpResponse::Ok().json(VersionInfo { version }))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Config
    let config = Config::from_env();

    // Initialize logging
    let env = env_logger::Env::new().filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    // Create appdata instance
    let instance = AppData::new(&config.data_path).await?;
    let appdata = web::Data::new(instance);

    // Create API server
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_header(header::CONTENT_TYPE);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::JsonConfig::default().limit(config.payload_limit))
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

    // Run API server
    server.run().await
}
