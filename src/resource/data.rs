use actix_web::{web, HttpResponse, Resource, HttpRequest};
use serde::Deserialize;

use lbasedb::dataset::Dataset;

use crate::utils::*;
use crate::error::JsonError;


#[derive(Debug, Deserialize)]
struct Query {
    feed: String,
    ix: Option<usize>,
    size: Option<usize>,
    col: Option<Vec<String>>,
}


async fn get_view(appdata: WebAppData, req: HttpRequest) -> APIResult {
    let query: Query = qs_parse(req.query_string())?;
    let ix = query.ix.ok_or(JsonError::from_str("ix required"))?;
    let size = query.size.ok_or(JsonError::from_str("size required"))?;
    let cols = query.col.unwrap_or(vec![]);
    let ds = appdata.db.data_get(&query.feed, ix, size, &cols).await?;
    Ok(HttpResponse::Ok().json(ds))
}


async fn push_view(appdata: WebAppData, query: web::Query<Query>, 
                   ds: web::Json<Dataset>) -> APIResult {
    appdata.db.data_push(&query.feed, &ds).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn save_view(appdata: WebAppData, query: web::Query<Query>, 
                   ds: web::Json<Dataset>) -> APIResult {
    let ix = query.ix.ok_or(JsonError::from_str("ix required"))?;
    appdata.db.data_save(&query.feed, ix, &ds).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn patch_view(appdata: WebAppData, query: web::Query<Query>, 
                    ds: web::Json<Dataset>) -> APIResult {
    let ix = query.ix.ok_or(JsonError::from_str("ix required"))?;
    appdata.db.data_patch(&query.feed, ix, &ds).await?;
    Ok(HttpResponse::NoContent().finish())
}


pub fn load_resource_data() -> Resource {
    web::resource("/data")
        .route(web::get().to(get_view))
        .route(web::post().to(push_view))
        .route(web::put().to(save_view))
        .route(web::patch().to(patch_view))
}
