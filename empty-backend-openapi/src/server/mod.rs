use futures::{future, future::BoxFuture, future::FutureExt, stream, stream::TryStreamExt, Stream};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use hyper::{Body, HeaderMap, Request, Response, StatusCode};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
use url::form_urlencoded;

use crate::header;
#[allow(unused_imports)]
use crate::models;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{
    Api, QuestionsGetResponse, QuestionsIdAnswersAnswerIdPutResponse,
    QuestionsIdAnswersGetResponse, QuestionsIdAnswersPatchResponse, QuestionsIdAnswersPostResponse,
    QuestionsIdDeleteResponse, QuestionsIdGetResponse, QuestionsIdPutResponse,
    QuestionsPostResponse,
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/v1/questions$",
            r"^/v1/questions/(?P<id>[^/?#]*)$",
            r"^/v1/questions/(?P<id>[^/?#]*)/answers/$",
            r"^/v1/questions/(?P<id>[^/?#]*)/answers/(?P<answerId>[^/?#]*)$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_QUESTIONS: usize = 0;
    pub(crate) static ID_QUESTIONS_ID: usize = 1;
    lazy_static! {
        pub static ref REGEX_QUESTIONS_ID: regex::Regex =
            regex::Regex::new(r"^/v1/questions/(?P<id>[^/?#]*)$")
                .expect("Unable to create regex for QUESTIONS_ID");
    }
    pub(crate) static ID_QUESTIONS_ID_ANSWERS_: usize = 2;
    lazy_static! {
        pub static ref REGEX_QUESTIONS_ID_ANSWERS_: regex::Regex =
            regex::Regex::new(r"^/v1/questions/(?P<id>[^/?#]*)/answers/$")
                .expect("Unable to create regex for QUESTIONS_ID_ANSWERS_");
    }
    pub(crate) static ID_QUESTIONS_ID_ANSWERS_ANSWERID: usize = 3;
    lazy_static! {
        pub static ref REGEX_QUESTIONS_ID_ANSWERS_ANSWERID: regex::Regex =
            regex::Regex::new(r"^/v1/questions/(?P<id>[^/?#]*)/answers/(?P<answerId>[^/?#]*)$")
                .expect("Unable to create regex for QUESTIONS_ID_ANSWERS_ANSWERID");
    }
}

pub struct MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(self.api_impl.clone()))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())
        .expect("Unable to create Method Not Allowed response"))
}

pub struct Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData,
        }
    }
}

impl<T, C> Clone for Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C>
where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Send + Sync + 'static,
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future {
        async fn run<T, C>(
            mut api_impl: T,
            req: (Request<Body>, C),
        ) -> Result<Response<Body>, crate::ServiceError>
        where
            T: Api<C> + Clone + Send + 'static,
            C: Has<XSpanIdString> + Send + Sync + 'static,
        {
            let (request, context) = req;
            let (parts, body) = request.into_parts();
            let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
            let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

            match &method {
                // QuestionsIdAnswersAnswerIdPut - PUT /questions/{id}/answers/{answerId}
                &hyper::Method::PUT if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_ANSWERID) => {
                    // Path parameters
                    let path: &str = &uri.path().to_string();
                    let path_params =
                    paths::REGEX_QUESTIONS_ID_ANSWERS_ANSWERID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE QUESTIONS_ID_ANSWERS_ANSWERID in set but failed match against \"{}\"", path, paths::REGEX_QUESTIONS_ID_ANSWERS_ANSWERID.as_str())
                    );

                    let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<i32>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let param_answer_id = match percent_encoding::percent_decode(path_params["answerId"].as_bytes()).decode_utf8() {
                    Ok(param_answer_id) => match param_answer_id.parse::<i32>() {
                        Ok(param_answer_id) => param_answer_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter answerId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["answerId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_answer: Option<models::Answer> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_answer) => param_answer,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.questions_id_answers_answer_id_put(
                                            param_id,
                                            param_answer_id,
                                            param_answer,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                QuestionsIdAnswersAnswerIdPutResponse::Ok
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Answer: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Answer")),
                        }
                }

                // QuestionsIdAnswersGet - GET /questions/{id}/answers/
                &hyper::Method::GET if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_) => {
                    // Path parameters
                    let path: &str = &uri.path().to_string();
                    let path_params =
                    paths::REGEX_QUESTIONS_ID_ANSWERS_
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE QUESTIONS_ID_ANSWERS_ in set but failed match against \"{}\"", path, paths::REGEX_QUESTIONS_ID_ANSWERS_.as_str())
                    );

                    let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<i32>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl.questions_id_answers_get(param_id, &context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .to_string()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            QuestionsIdAnswersGetResponse::Ok(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for QUESTIONS_ID_ANSWERS_GET_OK"));
                                let body = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body);
                            }
                        },
                        Err(_) => {
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // QuestionsIdAnswersPatch - PATCH /questions/{id}/answers/
                &hyper::Method::PATCH if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_) => {
                    // Path parameters
                    let path: &str = &uri.path().to_string();
                    let path_params =
                    paths::REGEX_QUESTIONS_ID_ANSWERS_
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE QUESTIONS_ID_ANSWERS_ in set but failed match against \"{}\"", path, paths::REGEX_QUESTIONS_ID_ANSWERS_.as_str())
                    );

                    let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<i32>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_answer: Option<Vec<models::Answer>> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_answer) => param_answer,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.questions_id_answers_patch(
                                            param_id,
                                            param_answer.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                QuestionsIdAnswersPatchResponse::Ok
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Answer: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Answer")),
                        }
                }

                // QuestionsIdAnswersPost - POST /questions/{id}/answers/
                &hyper::Method::POST if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_) => {
                    // Path parameters
                    let path: &str = &uri.path().to_string();
                    let path_params =
                    paths::REGEX_QUESTIONS_ID_ANSWERS_
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE QUESTIONS_ID_ANSWERS_ in set but failed match against \"{}\"", path, paths::REGEX_QUESTIONS_ID_ANSWERS_.as_str())
                    );

                    let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<i32>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_answer: Option<Vec<models::Answer>> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_answer) => param_answer,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.questions_id_answers_post(
                                            param_id,
                                            param_answer.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                QuestionsIdAnswersPostResponse::Ok
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Answer: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Answer")),
                        }
                }

                // QuestionsIdDelete - DELETE /questions/{id}
                &hyper::Method::DELETE if path.matched(paths::ID_QUESTIONS_ID) => {
                    // Path parameters
                    let path: &str = &uri.path().to_string();
                    let path_params =
                    paths::REGEX_QUESTIONS_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE QUESTIONS_ID in set but failed match against \"{}\"", path, paths::REGEX_QUESTIONS_ID.as_str())
                    );

                    let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<i32>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl.questions_id_delete(param_id, &context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .to_string()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            QuestionsIdDeleteResponse::Ok => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                            }
                        },
                        Err(_) => {
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // QuestionsIdGet - GET /questions/{id}
                &hyper::Method::GET if path.matched(paths::ID_QUESTIONS_ID) => {
                    // Path parameters
                    let path: &str = &uri.path().to_string();
                    let path_params =
                    paths::REGEX_QUESTIONS_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE QUESTIONS_ID in set but failed match against \"{}\"", path, paths::REGEX_QUESTIONS_ID.as_str())
                    );

                    let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<i32>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    let result = api_impl.questions_id_get(param_id, &context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .to_string()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => match rsp {
                            QuestionsIdGetResponse::Ok(body) => {
                                *response.status_mut() = StatusCode::from_u16(200)
                                    .expect("Unable to turn 200 into a StatusCode");
                                response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for QUESTIONS_ID_GET_OK"));
                                let body = serde_json::to_string(&body)
                                    .expect("impossible to fail to serialize");
                                *response.body_mut() = Body::from(body);
                            }
                        },
                        Err(_) => {
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // QuestionsIdPut - PUT /questions/{id}
                &hyper::Method::PUT if path.matched(paths::ID_QUESTIONS_ID) => {
                    // Path parameters
                    let path: &str = &uri.path().to_string();
                    let path_params =
                    paths::REGEX_QUESTIONS_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE QUESTIONS_ID in set but failed match against \"{}\"", path, paths::REGEX_QUESTIONS_ID.as_str())
                    );

                    let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<i32>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_question: Option<models::Question> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_question) => param_question,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.questions_id_put(
                                            param_id,
                                            param_question,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                QuestionsIdPutResponse::Ok
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Question: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Question")),
                        }
                }

                // QuestionsGet - GET /questions
                &hyper::Method::GET if path.matched(paths::ID_QUESTIONS) => {
                    let result = api_impl.questions_get(&context).await;
                    let mut response = Response::new(Body::empty());
                    response.headers_mut().insert(
                        HeaderName::from_static("x-span-id"),
                        HeaderValue::from_str(
                            (&context as &dyn Has<XSpanIdString>)
                                .get()
                                .0
                                .clone()
                                .to_string()
                                .as_str(),
                        )
                        .expect("Unable to create X-Span-ID header value"),
                    );

                    match result {
                        Ok(rsp) => {
                            match rsp {
                                QuestionsGetResponse::Ok(body) => {
                                    *response.status_mut() = StatusCode::from_u16(200)
                                        .expect("Unable to turn 200 into a StatusCode");
                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for QUESTIONS_GET_OK"));
                                    let body = serde_json::to_string(&body)
                                        .expect("impossible to fail to serialize");
                                    *response.body_mut() = Body::from(body);
                                }
                            }
                        }
                        Err(_) => {
                            // Application code returned an error. This should not happen, as the implementation should
                            // return a valid response.
                            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                            *response.body_mut() = Body::from("An internal error occurred");
                        }
                    }

                    Ok(response)
                }

                // QuestionsPost - POST /questions
                &hyper::Method::POST if path.matched(paths::ID_QUESTIONS) => {
                    // Body parameters (note that non-required body parameters will ignore garbage
                    // values, rather than causing a 400 response). Produce warning header and logs for
                    // any unused fields.
                    let result = body.into_raw().await;
                    match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_question: Option<models::Question> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_question) => param_question,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.questions_post(
                                            param_question,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                QuestionsPostResponse::Ok
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter Question: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter Question")),
                        }
                }

                _ if path.matched(paths::ID_QUESTIONS) => method_not_allowed(),
                _ if path.matched(paths::ID_QUESTIONS_ID) => method_not_allowed(),
                _ if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_) => method_not_allowed(),
                _ if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_ANSWERID) => method_not_allowed(),
                _ => Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response")),
            }
        }
        Box::pin(run(self.api_impl.clone(), req))
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // QuestionsIdAnswersAnswerIdPut - PUT /questions/{id}/answers/{answerId}
            &hyper::Method::PUT if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_ANSWERID) => {
                Some("QuestionsIdAnswersAnswerIdPut")
            }
            // QuestionsIdAnswersGet - GET /questions/{id}/answers/
            &hyper::Method::GET if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_) => {
                Some("QuestionsIdAnswersGet")
            }
            // QuestionsIdAnswersPatch - PATCH /questions/{id}/answers/
            &hyper::Method::PATCH if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_) => {
                Some("QuestionsIdAnswersPatch")
            }
            // QuestionsIdAnswersPost - POST /questions/{id}/answers/
            &hyper::Method::POST if path.matched(paths::ID_QUESTIONS_ID_ANSWERS_) => {
                Some("QuestionsIdAnswersPost")
            }
            // QuestionsIdDelete - DELETE /questions/{id}
            &hyper::Method::DELETE if path.matched(paths::ID_QUESTIONS_ID) => {
                Some("QuestionsIdDelete")
            }
            // QuestionsIdGet - GET /questions/{id}
            &hyper::Method::GET if path.matched(paths::ID_QUESTIONS_ID) => Some("QuestionsIdGet"),
            // QuestionsIdPut - PUT /questions/{id}
            &hyper::Method::PUT if path.matched(paths::ID_QUESTIONS_ID) => Some("QuestionsIdPut"),
            // QuestionsGet - GET /questions
            &hyper::Method::GET if path.matched(paths::ID_QUESTIONS) => Some("QuestionsGet"),
            // QuestionsPost - POST /questions
            &hyper::Method::POST if path.matched(paths::ID_QUESTIONS) => Some("QuestionsPost"),
            _ => None,
        }
    }
}
