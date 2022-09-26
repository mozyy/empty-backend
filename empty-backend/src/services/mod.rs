use actix_web::{http::Method, web, HttpResponse, Responder, Scope};

mod docs;
pub mod questions;
mod system;

trait Server {
    fn service(&self) -> Scope;
}

pub fn config_servers(cfg: &mut web::ServiceConfig) {
    cfg.service(questions::Server::new().service())
        .service(docs::Server::new().service())
        .default_service(web::to(default_handler));
}

async fn default_handler(req_method: Method) -> impl Responder {
    match req_method {
        Method::GET => HttpResponse::NotFound().body("not funddddd!"),
        _ => HttpResponse::MethodNotAllowed().finish(),
    }
}
