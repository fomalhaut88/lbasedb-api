mod config;
mod connection;
mod resource;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::config::Config;
use crate::connection::Connection;
use crate::resource::{load_resource_data, load_resource_feed, load_resource_col, 
                      load_resource_size};


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    let server = HttpServer::new(|| {
        let conn = Connection::new();
        let app_data = web::Data::new(conn);
        App::new()
            .app_data(app_data)
            .service(home)
            .service(load_resource_data())
            .service(load_resource_feed())
            .service(load_resource_col())
            .service(load_resource_size())
    })
        .workers(config.workers)
        .bind((config.host, config.port))?;

    server.run().await
}
