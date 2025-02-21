use actix_web::{web, HttpResponse, Resource};
use serde::{Serialize, Deserialize};

use crate::utils::*;


#[derive(Deserialize)]
struct Query {
    feed: String,
}


#[derive(Serialize, Deserialize)]
struct Item {
    size: usize,
}


async fn get_view(appdata: WebAppData, query: web::Query<Query>) -> APIResult {
    let db = &appdata.read().await.db;
    let item = Item {
        size: db.size_get(&query.feed).await?,
    };
    Ok(HttpResponse::Ok().json(item))
}


async fn save_view(appdata: WebAppData, query: web::Query<Query>, 
                   json: web::Json<Item>) -> APIResult {
    let db = &appdata.read().await.db;
    db.size_set(&query.feed, json.size).await?;
    Ok(HttpResponse::NoContent().finish())
}


pub fn load_resource_size() -> Resource {
    web::resource("/size")
        .route(web::get().to(get_view))
        .route(web::put().to(save_view))
}
