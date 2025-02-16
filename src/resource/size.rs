use actix_web::{web, HttpResponse, Resource};
use serde::Deserialize;

use crate::utils::*;


#[derive(Deserialize)]
struct Query {
    feed: String,
}


#[derive(Deserialize)]
struct Item {
    size: usize,
}


async fn get_view(appdata: WebAppData, query: web::Query<Query>) -> APIResult {
    let db = &appdata.lock().await.db;
    let body = format!("Size: {:?}", db.size_get(&query.feed)?);
    Ok(HttpResponse::Ok().body(body))
}


// async fn push_view() -> APIResult {
//     Ok(HttpResponse::Ok().body("Not implemented"))
// }


async fn save_view(appdata: WebAppData, query: web::Query<Query>, json: web::Json<Item>) -> APIResult {
    let db = &mut appdata.lock().await.db;
    db.size_set(&query.feed, json.size).await?;
    Ok(HttpResponse::NoContent().finish())
}


// async fn patch_view() -> APIResult {
//     Ok(HttpResponse::Ok().body("Not implemented"))
// }


// async fn delete_view() -> APIResult {
//     Ok(HttpResponse::Ok().body("Not implemented"))
// }


pub fn load_resource_size() -> Resource {
    web::resource("/size")
        .route(web::get().to(get_view))
        // .route(web::post().to(push_view))
        .route(web::put().to(save_view))
        // .route(web::patch().to(patch_view))
        // .route(web::delete().to(delete_view))
}
