use std::time::Duration;
pub mod timestamp;
pub mod uuid;

use tonic::transport::Server;
use tower::{
    layer::util::{Identity, Stack},
    timeout::TimeoutLayer,
    ServiceBuilder,
};
use tower_http::{
    classify::{GrpcErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};

pub type Resp<T> = core::result::Result<tonic::Response<T>, tonic::Status>;

pub struct Response<T>(pub T);

impl<T> Response<T> {
    pub fn new(value: T) -> Self {
        Response(value)
    }
}

impl<T> From<Response<T>> for Resp<T> {
    fn from(value: Response<T>) -> Self {
        Ok(tonic::Response::new(value.0))
    }
}
// impl<T> std::ops::Try for Response<T> {
//     type Output = Resp<T>;

//     type Residual = Error;

//     fn from_output(output: Self::Output) -> Self {
//         todo!()
//     }

//     fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
//         todo!()
//     }
// }

pub fn server() -> Server<
    Stack<
        Stack<TraceLayer<SharedClassifier<GrpcErrorsAsFailures>>, Stack<TimeoutLayer, Identity>>,
        Identity,
    >,
> {
    // Build our middleware stack
    let layer = ServiceBuilder::new()
        // Set a timeout
        .timeout(Duration::from_secs(10))
        // Compress responses
        // .layer(CompressionLayer::new())
        // Mark the `Authorization` header as sensitive so it doesn't show in logs
        // .layer(SetSensitiveHeadersLayer::new(once(header::AUTHORIZATION)))
        // Log all requests and responses
        .layer(
            tower_http::trace::TraceLayer::new_for_grpc(), // .on_request(DefaultMakeSpan::new().include_headers(true)),
        )
        .into_inner();

    let server = Server::builder().layer(layer);
    server
}
