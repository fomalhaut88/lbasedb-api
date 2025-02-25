use actix_web::{web, HttpResponse, Resource};
use serde::{Serialize, Deserialize};

use crate::utils::*;


#[derive(Deserialize)]
struct Query {
    feed: String,
    name: Option<String>,
}


#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    datatype: Option<String>,
}


async fn get_view(appdata: WebAppData, query: web::Query<Query>) -> APIResult {
    let items = appdata.db.col_list(&query.feed).await?.iter()
        .map(|col_item| Item {
            name: col_item.get_name(), 
            datatype: Some(col_item.get_datatype()),
        }).collect::<Vec<Item>>();
    Ok(HttpResponse::Ok().json(items))
}


async fn push_view(appdata: WebAppData, query: web::Query<Query>, 
                   json: web::Json<Item>) -> APIResult {
    appdata.db.col_add(&query.feed, &json.name, 
                       &json.datatype.clone().unwrap()).await?;
    Ok(HttpResponse::Created().finish())
}


async fn patch_view(appdata: WebAppData, query: web::Query<Query>, 
                    json: web::Json<Item>) -> APIResult {
    appdata.db.col_rename(&query.feed, &query.name.clone().unwrap(), 
                          &json.name).await?;
    Ok(HttpResponse::NoContent().finish())
}


async fn delete_view(appdata: WebAppData, 
                     query: web::Query<Query>) -> APIResult {
    appdata.db.col_remove(&query.feed, &query.name.clone().unwrap()).await?;
    Ok(HttpResponse::NoContent().finish())
}


pub fn load_resource_col() -> Resource {
    web::resource("/col")
        .route(web::get().to(get_view))
        .route(web::post().to(push_view))
        .route(web::patch().to(patch_view))
        .route(web::delete().to(delete_view))
}
