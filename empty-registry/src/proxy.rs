use crate::registry::model::Registry;
use axum::{
    extract::{Path, State},
    http::{uri::Uri, Request, Response},
};
use hyper::{client::HttpConnector, Body};
use std::sync::{Arc, Mutex};
use std::time::Duration;
type Client = hyper::client::Client<HttpConnector, Body>;
use std::time::Instant;
#[derive(Clone)]
pub struct Proxy {
    pub registry: Arc<Mutex<Registry>>,
    pub client: Client,
}

impl Default for Proxy {
    fn default() -> Self {
        Self {
            registry: Default::default(),
            client: hyper::Client::builder()
                .pool_idle_timeout(Duration::from_secs(30))
                .http2_only(true)
                .build_http(),
        }
    }
}

impl Proxy {
    pub fn new(registry: Arc<Mutex<Registry>>) -> Self {
        Self {
            registry,
            client: hyper::Client::builder().http2_only(true).build_http(),
        }
    }
    pub async fn handler(
        Path((service, _rest)): Path<(String, String)>,
        State(proxy): State<Proxy>,
        mut req: Request<Body>,
    ) -> Response<Body> {
        let start = Instant::now();
        let path = req.uri().path();
        let path_query = req
            .uri()
            .path_and_query()
            .map(|v| v.as_str())
            .unwrap_or(path);
        let services = {
            let mut registry = proxy.registry.lock().unwrap();
            registry.list_service(service).unwrap()
        };
        let endpoint = services.get(0).unwrap().endpoint.clone();

        let uri = format!("http://{}{}", endpoint, path_query);

        *req.uri_mut() = Uri::try_from(uri).unwrap();
        // let client = hyper::Client::builder().http2_only(true).build_http();
        let second = Instant::now();
        let resp = proxy.client.request(req).await.unwrap();
        let end = Instant::now();
        log::info!(
            "proxy time: service:{:?}, client:{:?}, resp: {:?}",
            second.duration_since(start),
            end.duration_since(second),
            resp
        );
        resp
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
