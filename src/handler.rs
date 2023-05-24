use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::SqlitePool;

use crate::{
    db::{calculate_tolls, get_categories, get_directions, get_enters, get_exits},
    model::{ApiErrors, EntersRequest, ExitsRequest, TollsRequest},
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
    let result = get_exits(request.0, &db);
    match result.await {
        Ok(exits_result) => HttpResponse::Ok().json(exits_result),
        Err(e) => match e {
            ApiErrors::ValidationError(msg) => HttpResponse::BadRequest().body(msg).into(),
            ApiErrors::SqlError(msg) => HttpResponse::InternalServerError().body(msg).into(),
        },
    }
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

#[post("/tolls")]
async fn tolls(request: web::Json<TollsRequest>, db: web::Data<SqlitePool>) -> impl Responder {
    let result = calculate_tolls(request.0, &db);
    match result.await {
        Ok(tolls_result) => HttpResponse::Ok().json(tolls_result),
        Err(e) => match e {
            ApiErrors::ValidationError(msg) => HttpResponse::BadRequest().body(msg).into(),
            ApiErrors::SqlError(msg) => HttpResponse::InternalServerError().body(msg).into(),
        },
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(version)
        .service(enters)
        .service(directions)
        .service(categories)
        .service(tolls)
        .service(exits);

    conf.service(scope);
}
