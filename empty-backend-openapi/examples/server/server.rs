//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use openapi_client::{
    Api,
    QuestionsIdAnswersAnswerIdPutResponse,
    QuestionsIdAnswersGetResponse,
    QuestionsIdAnswersPatchResponse,
    QuestionsIdAnswersPostResponse,
    QuestionsIdDeleteResponse,
    QuestionsIdGetResponse,
    QuestionsIdPutResponse,
    QuestionsGetResponse,
    QuestionsPostResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    async fn questions_id_answers_answer_id_put(
        &self,
        id: i32,
        answer_id: i32,
        answer: Option<models::Answer>,
        context: &C) -> Result<QuestionsIdAnswersAnswerIdPutResponse, ApiError>
    {
        let context = context.clone();
        info!("questions_id_answers_answer_id_put({}, {}, {:?}) - X-Span-ID: {:?}", id, answer_id, answer, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn questions_id_answers_get(
        &self,
        id: i32,
        context: &C) -> Result<QuestionsIdAnswersGetResponse, ApiError>
    {
        let context = context.clone();
        info!("questions_id_answers_get({}) - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn questions_id_answers_patch(
        &self,
        id: i32,
        answer: Option<&Vec<models::Answer>>,
        context: &C) -> Result<QuestionsIdAnswersPatchResponse, ApiError>
    {
        let context = context.clone();
        info!("questions_id_answers_patch({}, {:?}) - X-Span-ID: {:?}", id, answer, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn questions_id_answers_post(
        &self,
        id: i32,
        answer: Option<&Vec<models::Answer>>,
        context: &C) -> Result<QuestionsIdAnswersPostResponse, ApiError>
    {
        let context = context.clone();
        info!("questions_id_answers_post({}, {:?}) - X-Span-ID: {:?}", id, answer, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn questions_id_delete(
        &self,
        id: i32,
        context: &C) -> Result<QuestionsIdDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("questions_id_delete({}) - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn questions_id_get(
        &self,
        id: i32,
        context: &C) -> Result<QuestionsIdGetResponse, ApiError>
    {
        let context = context.clone();
        info!("questions_id_get({}) - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn questions_id_put(
        &self,
        id: i32,
        question: Option<models::Question>,
        context: &C) -> Result<QuestionsIdPutResponse, ApiError>
    {
        let context = context.clone();
        info!("questions_id_put({}, {:?}) - X-Span-ID: {:?}", id, question, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn questions_get(
        &self,
        context: &C) -> Result<QuestionsGetResponse, ApiError>
    {
        let context = context.clone();
        info!("questions_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn questions_post(
        &self,
        question: Option<models::Question>,
        context: &C) -> Result<QuestionsPostResponse, ApiError>
    {
        let context = context.clone();
        info!("questions_post({:?}) - X-Span-ID: {:?}", question, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
