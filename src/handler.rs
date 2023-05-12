use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{
    db::{get_categories, get_directions, get_enters, get_exits},
    model::{EntersRequest, ExitsRequest},
};

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"version": "1.0"}))
}

#[post("/enters")]
async fn enters(request: web::Json<EntersRequest>, db: web::Data<SqlitePool>) -> impl Responder {
    let result = get_enters(request.direction, &db);
    HttpResponse::Ok().json(result.await.unwrap())
}

#[post("/exits")]
async fn exits(request: web::Json<ExitsRequest>, db: web::Data<SqlitePool>) -> impl Responder {
    let result = get_exits(request.direction, &db);
    HttpResponse::Ok().json(result.await.unwrap())
}

#[get("/directions")]
async fn directions(db: web::Data<SqlitePool>) -> impl Responder {
    let result = get_directions(&db);
    HttpResponse::Ok().json(result.await.unwrap())
}

#[get("/categories")]
async fn categories(db: web::Data<SqlitePool>) -> impl Responder {
    let result = get_categories(&db);
    HttpResponse::Ok().json(result.await.unwrap())
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(version)
        .service(enters)
        .service(directions)
        .service(categories)
        .service(exits);

    conf.service(scope);
}
