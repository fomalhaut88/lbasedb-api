use actix_web::{web, HttpResponse, Responder, Resource};


async fn get_view() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}


async fn push_view() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}


async fn save_view() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}


async fn patch_view() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}


async fn delete_view() -> impl Responder {
    HttpResponse::Ok().body("Not implemented")
}


pub fn load_resource_size() -> Resource {
    web::resource("/size")
        .route(web::get().to(get_view))
        .route(web::post().to(push_view))
        .route(web::put().to(save_view))
        .route(web::patch().to(patch_view))
        .route(web::delete().to(delete_view))
}
