use actix_web::{web, HttpResponse, Resource};
use serde::{Serialize, Deserialize};

use crate::utils::*;


#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
}


async fn get_view(appdata: WebAppData) -> APIResult {
    let db = &appdata.read().await.db;
    let items = db.feed_list().await.iter()
        .map(|feed_item| Item { name: feed_item.get_name() })
        .collect::<Vec<Item>>();
    Ok(HttpResponse::Ok().json(items))
}


async fn push_view(appdata: WebAppData, json: web::Json<Item>) -> APIResult {
    let db = &appdata.write().await.db;
    db.feed_add(&json.name).await?;
    Ok(HttpResponse::Created().finish())
}


async fn patch_view(appdata: WebAppData, query: web::Query<Item>, 
                    json: web::Json<Item>) -> APIResult {
    let db = &appdata.write().await.db;
    db.feed_rename(&query.name, &json.name).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn delete_view(appdata: WebAppData, 
                     query: web::Query<Item>) -> APIResult {
    let db = &appdata.write().await.db;
    db.feed_remove(&query.name).await?;
    Ok(HttpResponse::NoContent().finish())
}


pub fn load_resource_feed() -> Resource {
    web::resource("/feed")
        .route(web::get().to(get_view))
        .route(web::post().to(push_view))
        .route(web::patch().to(patch_view))
        .route(web::delete().to(delete_view))
}
