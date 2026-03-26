use actix_web::{web, HttpResponse, Resource, HttpRequest};
use serde::Deserialize;

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
    let ix = query.ix.ok_or(JsonError::new("param required", "ix"))?;
    let size = query.size.ok_or(JsonError::new("param required", "size"))?;
    let cols = query.col.ok_or(JsonError::new("param required", "col[]"))?;
    let buffer: Vec<u8> = appdata.db.bytes_get(&query.feed, &cols, ix,
                                               size).await?;
    Ok(HttpResponse::Ok().body(buffer))
}


async fn push_view(appdata: WebAppData, req: HttpRequest,
                   body: web::Bytes) -> APIResult {
    let query: Query = qs_parse(req.query_string())?;
    let cols = query.col.ok_or(JsonError::new("param required", "col[]"))?;
    appdata.db.bytes_push(&query.feed, &cols, body.as_ref()).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn save_view(appdata: WebAppData, req: HttpRequest,
                   body: web::Bytes) -> APIResult {
    let query: Query = qs_parse(req.query_string())?;
    let ix = query.ix.ok_or(JsonError::new("param required", "ix"))?;
    let cols = query.col.ok_or(JsonError::new("param required", "col[]"))?;
    appdata.db.bytes_save(&query.feed, &cols, ix, body.as_ref()).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn patch_view(appdata: WebAppData, req: HttpRequest,
                   body: web::Bytes) -> APIResult {
    let query: Query = qs_parse(req.query_string())?;
    let ix = query.ix.ok_or(JsonError::new("param required", "ix"))?;
    let cols = query.col.ok_or(JsonError::new("param required", "col[]"))?;
    appdata.db.bytes_patch(&query.feed, &cols, ix, body.as_ref()).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn delete_view(appdata: WebAppData, req: HttpRequest) -> APIResult {
    let query: Query = qs_parse(req.query_string())?;
    let ix = query.ix.ok_or(JsonError::new("param required", "ix"))?;
    let size = query.size.ok_or(JsonError::new("param required", "size"))?;
    let cols = query.col.ok_or(JsonError::new("param required", "col[]"))?;
    appdata.db.bytes_reset(&query.feed, &cols, ix, size).await?;
    Ok(HttpResponse::NoContent().finish())
}


pub fn load_resource_bytes() -> Resource {
    web::resource("/bytes")
        .route(web::get().to(get_view))
        .route(web::post().to(push_view))
        .route(web::put().to(save_view))
        .route(web::patch().to(patch_view))
        .route(web::delete().to(delete_view))
}
