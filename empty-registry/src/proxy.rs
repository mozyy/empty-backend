use crate::registry::Registry;
use axum::{
    extract::{Path, State},
    http::{uri::Uri, Request, Response},
    routing::post,
    Router,
};
use hyper::{client::HttpConnector, Body};
use std::sync::{Arc, Mutex};

type Client = hyper::client::Client<HttpConnector, Body>;

#[derive(Default, Clone)]
pub struct Proxy {
    pub registry: Arc<Mutex<Registry>>,
    pub client: Client,
}

impl Proxy {
    pub async fn  handler(
        Path(service): Path<String>,
        State(proxy): State<Proxy>,
        mut req: Request<Body>,
    ) -> Response<Body> {
        log::info!("proxy req: {:?}",req);
        let path = req.uri().path();
        let path_query = req
            .uri()
            .path_and_query()
            .map(|v| v.as_str())
            .unwrap_or(path);
        let endpoint = {
            let mut registry = proxy.registry.lock().unwrap();
            let services = registry.list_service(service).unwrap();
            log::info!("services: {:?}", services);
            services.get(0).unwrap().endpoint.clone()
        };

        let uri = format!("http://{}{}", endpoint, path_query);

        *req.uri_mut() = Uri::try_from(uri).unwrap();

        proxy.client.request(req).await.unwrap()
    }
}

pub async fn handler(
    Path(service): Path<String>,
    State(proxy): State<Proxy>,
    mut req: Request<Body>,
) -> Response<Body> {
    log::info!("proxy request: {:?}", req);
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    let endpoint = {
        let mut registry = proxy.registry.lock().unwrap();
        let services = registry.list_service(service).unwrap();
        log::info!("services: {:?}", services);
        services.get(0).unwrap().endpoint.clone()
    };

    let uri = format!("http://{}{}", endpoint, path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    proxy.client.request(req).await.unwrap()
}
