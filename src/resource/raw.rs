use actix_web::{web, HttpResponse, Resource};
use serde::{Serialize, Deserialize};

use crate::utils::*;
use crate::error::JsonError;


#[derive(Debug, Serialize, Deserialize)]
struct Query {
    feed: String,
    col: String,
    ix: usize,
    size: Option<usize>,
}


async fn get_view(appdata: WebAppData, query: web::Query<Query>) -> APIResult {
    let size = query.size.ok_or(JsonError::from_str("size required"))?;
    let block = appdata.db.raw_get(&query.feed, &query.col, query.ix, 
                                   size).await?;
    Ok(HttpResponse::Ok().body(block))
}


async fn update_view(appdata: WebAppData, query: web::Query<Query>, 
                     payload: web::Bytes) -> APIResult {
    appdata.db.raw_set(
        &query.feed,
        &query.col,
        query.ix,
        &payload
    ).await?;
    Ok(HttpResponse::NoContent().finish())
}


pub fn load_resource_raw() -> Resource {
    web::resource("/raw")
        .route(web::get().to(get_view))
        .route(web::post().to(update_view))
}
