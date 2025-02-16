use actix_web::{web, HttpResponse, Resource};
use serde::{Serialize, Deserialize};

use crate::utils::*;


#[derive(Deserialize)]
struct Query {
    feed: String,
    ix: Option<u64>,
    size: Option<u64>,
    col: Option<Vec<String>>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum DataUnit {
    I(i64),
    F(f64),
    S(String),
}


type DatasetItem = std::collections::HashMap<String, Vec<DataUnit>>;


async fn get_view(appdata: WebAppData) -> APIResult {
    // let db = &appdata.lock().await.db;
    // let body = format!("Cols: {:?}", db.col_list(&query.feed)?);
    // Ok(HttpResponse::Ok().body(body))
    Ok(HttpResponse::Ok().body("yes"))
}


async fn push_view(appdata: WebAppData, json: web::Json<DatasetItem>) -> APIResult {
    println!("{:?}", json);
    // let db = &mut appdata.lock().await.db;
    // db.col_add(&query.feed, &json.name, &json.datatype.clone().unwrap()).await?;
    // Ok(HttpResponse::Created().finish())
    Ok(HttpResponse::Ok().body("yes"))
}


async fn save_view(appdata: WebAppData) -> APIResult {
    // let db = &mut appdata.lock().await.db;
    // db.feed_rename(&query.name, &json.name).await?;
    // Ok(HttpResponse::Forbidden().body("Data cannot be deleted"))
    Ok(HttpResponse::Ok().body("yes"))
}


async fn patch_view(appdata: WebAppData) -> APIResult {
    // let db = &mut appdata.lock().await.db;
    // db.col_rename(&query.feed, &query.name.clone().unwrap(), &json.name).await?;
    // Ok(HttpResponse::NoContent().finish())
    Ok(HttpResponse::Ok().body("yes"))
}


// async fn delete_view(appdata: WebAppData) -> APIResult {
//     let db = &mut appdata.lock().await.db;
//     db.col_remove(&query.feed, &query.name.clone().unwrap()).await?;
//     Ok(HttpResponse::NoContent().finish())
// }


// async fn get_view(appdata: web::Data<AppData>) -> impl Responder {
//     let body = format!("Yes get: 25");
//     HttpResponse::Ok().body(body)
// }


// async fn push_view(appdata: web::Data<AppData>) -> impl Responder {
//     let body = format!("Yes push: 25");
//     HttpResponse::Ok().body(body)
// }


// async fn save_view(appdata: web::Data<AppData>) -> impl Responder {
//     let body = format!("Yes save: 25");
//     HttpResponse::Ok().body(body)
// }


// async fn patch_view(appdata: web::Data<AppData>) -> impl Responder {
//     let body = format!("Yes patch: 25");
//     HttpResponse::Ok().body(body)
// }


// async fn delete_view() -> impl Responder {
//     HttpResponse::Forbidden().body("Data cannot be deleted")
// }


pub fn load_resource_data() -> Resource {
    web::resource("/data")
        .route(web::get().to(get_view))
        .route(web::post().to(push_view))
        .route(web::put().to(save_view))
        .route(web::patch().to(patch_view))
        // .route(web::delete().to(delete_view))
}
