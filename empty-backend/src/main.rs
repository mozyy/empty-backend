use std::{env, error::Error, net::Ipv4Addr};

use actix_web::middleware::{Compress, NormalizePath};
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::Connection;
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

mod services;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    info!("starting HTTP server at http://localhost:8080");
    // set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Compress::default())
            // .wrap(NormalizePath::default())
            .configure(services::config_servers)
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
