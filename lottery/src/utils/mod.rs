use http::status;
use std::future::Future;
use tonic::body::BoxBody;
use tower::{Layer, Service};
pub mod diesel;
pub mod wx;

#[derive(Clone)]
pub struct AuthLayer {}

impl<S> Layer<S> for AuthLayer
where
    S: Clone,
{
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service { inner }
    }
}

#[derive(Clone)]
pub struct AuthService<S> {
    inner: S,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

impl<S, ReqBody> Service<http::Request<ReqBody>> for AuthService<S>
where
    S: Service<http::Request<ReqBody>, Error = Error> + Clone,
{
    type Response = http::Response<BoxBody>;

    type Error = Error;

    type Future = AuthFuture<S>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<ReqBody>) -> Self::Future {
        AuthFuture {
            service: self.inner.clone(),
        }
    }
}

pub struct AuthFuture<S> {
    service: S,
}

impl<S> Future for AuthFuture<S> {
    type Output = Result<http::Response<BoxBody>, Error>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}
