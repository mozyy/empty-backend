use std::{error::Error, net::Ipv4Addr};

use actix_web::middleware::Compress;
use actix_web::{middleware::Logger, web, App, HttpServer};
use empty_utils::diesel::db;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    println!("starting HTTP server at http://localhost:8080");
    // set up database connection pool
    let pool = db::DbPool::default();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Compress::default())
        // .wrap(NormalizePath::default())
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
