#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]
#![allow(unused_imports)]

use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::task::{Context, Poll};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "/v1";
pub const API_VERSION: &str = "1.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionsIdAnswersAnswerIdPutResponse {
    /// ok
    Ok,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionsIdAnswersGetResponse {
    /// ok
    Ok(Vec<models::Answer>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionsIdAnswersPatchResponse {
    /// ok
    Ok,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionsIdAnswersPostResponse {
    /// ok
    Ok,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionsIdDeleteResponse {
    /// ok
    Ok,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionsIdGetResponse {
    /// ok
    Ok(models::Question),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionsIdPutResponse {
    /// ok
    Ok,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionsGetResponse {
    /// ok
    Ok(Vec<models::Question>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionsPostResponse {
    /// ok
    Ok,
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    async fn questions_id_answers_answer_id_put(
        &self,
        id: i32,
        answer_id: i32,
        answer: Option<models::Answer>,
        context: &C,
    ) -> Result<QuestionsIdAnswersAnswerIdPutResponse, ApiError>;

    async fn questions_id_answers_get(
        &self,
        id: i32,
        context: &C,
    ) -> Result<QuestionsIdAnswersGetResponse, ApiError>;

    async fn questions_id_answers_patch(
        &self,
        id: i32,
        answer: Option<&Vec<models::Answer>>,
        context: &C,
    ) -> Result<QuestionsIdAnswersPatchResponse, ApiError>;

    async fn questions_id_answers_post(
        &self,
        id: i32,
        answer: Option<&Vec<models::Answer>>,
        context: &C,
    ) -> Result<QuestionsIdAnswersPostResponse, ApiError>;

    async fn questions_id_delete(
        &self,
        id: i32,
        context: &C,
    ) -> Result<QuestionsIdDeleteResponse, ApiError>;

    async fn questions_id_get(
        &self,
        id: i32,
        context: &C,
    ) -> Result<QuestionsIdGetResponse, ApiError>;

    async fn questions_id_put(
        &self,
        id: i32,
        question: Option<models::Question>,
        context: &C,
    ) -> Result<QuestionsIdPutResponse, ApiError>;

    async fn questions_get(&self, context: &C) -> Result<QuestionsGetResponse, ApiError>;

    async fn questions_post(
        &self,
        question: Option<models::Question>,
        context: &C,
    ) -> Result<QuestionsPostResponse, ApiError>;
}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    async fn questions_id_answers_answer_id_put(
        &self,
        id: i32,
        answer_id: i32,
        answer: Option<models::Answer>,
    ) -> Result<QuestionsIdAnswersAnswerIdPutResponse, ApiError>;

    async fn questions_id_answers_get(
        &self,
        id: i32,
    ) -> Result<QuestionsIdAnswersGetResponse, ApiError>;

    async fn questions_id_answers_patch(
        &self,
        id: i32,
        answer: Option<&Vec<models::Answer>>,
    ) -> Result<QuestionsIdAnswersPatchResponse, ApiError>;

    async fn questions_id_answers_post(
        &self,
        id: i32,
        answer: Option<&Vec<models::Answer>>,
    ) -> Result<QuestionsIdAnswersPostResponse, ApiError>;

    async fn questions_id_delete(&self, id: i32) -> Result<QuestionsIdDeleteResponse, ApiError>;

    async fn questions_id_get(&self, id: i32) -> Result<QuestionsIdGetResponse, ApiError>;

    async fn questions_id_put(
        &self,
        id: i32,
        question: Option<models::Question>,
    ) -> Result<QuestionsIdPutResponse, ApiError>;

    async fn questions_get(&self) -> Result<QuestionsGetResponse, ApiError>;

    async fn questions_post(
        &self,
        question: Option<models::Question>,
    ) -> Result<QuestionsPostResponse, ApiError>;
}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync>
where
    Self: Sized,
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
        ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    async fn questions_id_answers_answer_id_put(
        &self,
        id: i32,
        answer_id: i32,
        answer: Option<models::Answer>,
    ) -> Result<QuestionsIdAnswersAnswerIdPutResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .questions_id_answers_answer_id_put(id, answer_id, answer, &context)
            .await
    }

    async fn questions_id_answers_get(
        &self,
        id: i32,
    ) -> Result<QuestionsIdAnswersGetResponse, ApiError> {
        let context = self.context().clone();
        self.api().questions_id_answers_get(id, &context).await
    }

    async fn questions_id_answers_patch(
        &self,
        id: i32,
        answer: Option<&Vec<models::Answer>>,
    ) -> Result<QuestionsIdAnswersPatchResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .questions_id_answers_patch(id, answer, &context)
            .await
    }

    async fn questions_id_answers_post(
        &self,
        id: i32,
        answer: Option<&Vec<models::Answer>>,
    ) -> Result<QuestionsIdAnswersPostResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .questions_id_answers_post(id, answer, &context)
            .await
    }

    async fn questions_id_delete(&self, id: i32) -> Result<QuestionsIdDeleteResponse, ApiError> {
        let context = self.context().clone();
        self.api().questions_id_delete(id, &context).await
    }

    async fn questions_id_get(&self, id: i32) -> Result<QuestionsIdGetResponse, ApiError> {
        let context = self.context().clone();
        self.api().questions_id_get(id, &context).await
    }

    async fn questions_id_put(
        &self,
        id: i32,
        question: Option<models::Question>,
    ) -> Result<QuestionsIdPutResponse, ApiError> {
        let context = self.context().clone();
        self.api().questions_id_put(id, question, &context).await
    }

    async fn questions_get(&self) -> Result<QuestionsGetResponse, ApiError> {
        let context = self.context().clone();
        self.api().questions_get(&context).await
    }

    async fn questions_post(
        &self,
        question: Option<models::Question>,
    ) -> Result<QuestionsPostResponse, ApiError> {
        let context = self.context().clone();
        self.api().questions_post(question, &context).await
    }
}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
