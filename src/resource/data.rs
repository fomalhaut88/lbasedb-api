use actix_web::{web, HttpResponse, Resource, HttpRequest};
use serde::{Serialize, Deserialize};
use serde_qs;

use lbase2::dataset::Dataset;

use crate::utils::*;


#[derive(Debug, Serialize, Deserialize)]
struct Query {
    feed: String,
    ix: Option<usize>,
    size: Option<usize>,
    col: Option<Vec<String>>,
}


async fn get_view(appdata: WebAppData, req: HttpRequest) -> APIResult {
    let query = serde_qs::from_str::<Query>(&req.query_string()).unwrap();
    let db = &mut appdata.lock().await.db;
    let ds = db.data_get(
        &query.feed,
        query.ix.unwrap(),
        query.size.unwrap(),
        &query.col.unwrap_or(vec![]),
    ).await?;
    Ok(HttpResponse::Ok().json(ds))
}


// async fn get_view(appdata: WebAppData, query: web::Query<Query>) -> APIResult {
//     let db = &mut appdata.lock().await.db;
//     let ds = db.data_get(
//         &query.feed,
//         query.ix.unwrap(),
//         query.size.unwrap(),
//         &query.col.clone().unwrap_or(vec![]),
//     ).await?;
//     Ok(HttpResponse::Ok().json(ds))
// }


async fn push_view(appdata: WebAppData, query: web::Query<Query>, ds: web::Json<Dataset>) -> APIResult {
    let db = &mut appdata.lock().await.db;
    db.data_push(&query.feed, &ds).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn save_view(appdata: WebAppData, query: web::Query<Query>, ds: web::Json<Dataset>) -> APIResult {
    let db = &mut appdata.lock().await.db;
    db.data_save(&query.feed, query.ix.unwrap(), &ds).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn patch_view(appdata: WebAppData, query: web::Query<Query>, ds: web::Json<Dataset>) -> APIResult {
    let db = &mut appdata.lock().await.db;
    db.data_patch(&query.feed, query.ix.unwrap(), &ds).await?;
    Ok(HttpResponse::NoContent().finish())
}


pub fn load_resource_data() -> Resource {
    web::resource("/data")
        .route(web::get().to(get_view))
        .route(web::post().to(push_view))
        .route(web::put().to(save_view))
        .route(web::patch().to(patch_view))
}
