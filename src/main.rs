mod db;
mod handler;
mod model;

use std::{env, io};

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};

use sqlx::SqlitePool;

const DB_URL: &str = "sqlite://tolls.db3";

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    env_logger::init();

    let db = match SqlitePool::connect(DB_URL).await {
        Ok(db) => {
            println!("✅ Connection to the database is successful!");
            db
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173, https://opass.dsw.mywire.org")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT]);
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(handler::config)
            .wrap(cors)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
