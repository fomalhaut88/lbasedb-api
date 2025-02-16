use actix_web::{web, HttpResponse, Resource};
use serde::Deserialize;

use crate::utils::*;


#[derive(Deserialize)]
struct Item {
    name: String,
}


async fn get_view(appdata: WebAppData) -> APIResult {
    let db = &appdata.lock().await.db;
    let body = format!("Feeds: {:?}", db.feed_list());
    Ok(HttpResponse::Ok().body(body))
}


async fn push_view(appdata: WebAppData, json: web::Json<Item>) -> APIResult {
    let db = &mut appdata.lock().await.db;
    db.feed_add(&json.name).await?;
    Ok(HttpResponse::Created().finish())
}


// async fn save_view(appdata: WebAppData, query: web::Query<Item>, json: web::Json<Item>) -> APIResult {
//     let db = &mut appdata.lock().await.db;
//     db.feed_rename(&query.name, &json.name).await?;
//     Ok(HttpResponse::NoContent().finish())
// }


async fn patch_view(appdata: WebAppData, query: web::Query<Item>, json: web::Json<Item>) -> APIResult {
    let db = &mut appdata.lock().await.db;
    db.feed_rename(&query.name, &json.name).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn delete_view(appdata: WebAppData, query: web::Query<Item>) -> APIResult {
    let db = &mut appdata.lock().await.db;
    db.feed_remove(&query.name).await?;
    Ok(HttpResponse::NoContent().finish())
}


pub fn load_resource_feed() -> Resource {
    web::resource("/feed")
        .route(web::get().to(get_view))
        .route(web::post().to(push_view))
        // .route(web::put().to(save_view))
        .route(web::patch().to(patch_view))
        .route(web::delete().to(delete_view))
}
