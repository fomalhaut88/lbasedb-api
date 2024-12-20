use actix_web::{web, HttpResponse, Responder, Resource};

use crate::connection::Connection;


async fn get_view(conn: web::Data<Connection>) -> impl Responder {
    let body = format!("Yes get: {}", conn.x);
    HttpResponse::Ok().body(body)
}


async fn push_view(conn: web::Data<Connection>) -> impl Responder {
    let body = format!("Yes push: {}", conn.x);
    HttpResponse::Ok().body(body)
}


async fn save_view(conn: web::Data<Connection>) -> impl Responder {
    let body = format!("Yes save: {}", conn.x);
    HttpResponse::Ok().body(body)
}


async fn patch_view(conn: web::Data<Connection>) -> impl Responder {
    let body = format!("Yes patch: {}", conn.x);
    HttpResponse::Ok().body(body)
}


async fn delete_view() -> impl Responder {
    HttpResponse::Forbidden().body("Data cannot be deleted")
}


pub fn load_resource_data() -> Resource {
    web::resource("/data")
        .route(web::get().to(get_view))
        .route(web::post().to(push_view))
        .route(web::put().to(save_view))
        .route(web::patch().to(patch_view))
        .route(web::delete().to(delete_view))
}
