#![allow(missing_docs, unused_variables, trivial_casts)]

use clap::{App, Arg};
#[allow(unused_imports)]
use futures::{future, stream, Stream};
#[allow(unused_imports)]
use openapi_client::{
    models, Api, ApiNoContext, Client, ContextWrapperExt, QuestionsGetResponse,
    QuestionsIdAnswersAnswerIdPutResponse, QuestionsIdAnswersGetResponse,
    QuestionsIdAnswersPatchResponse, QuestionsIdAnswersPostResponse, QuestionsIdDeleteResponse,
    QuestionsIdGetResponse, QuestionsIdPutResponse, QuestionsPostResponse,
};

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(
            Arg::with_name("operation")
                .help("Sets the operation to run")
                .possible_values(&[
                    "QuestionsIdAnswersAnswerIdPut",
                    "QuestionsIdAnswersGet",
                    "QuestionsIdAnswersPatch",
                    "QuestionsIdAnswersPost",
                    "QuestionsIdDelete",
                    "QuestionsIdGet",
                    "QuestionsIdPut",
                    "QuestionsGet",
                    "QuestionsPost",
                ])
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("https")
                .long("https")
                .help("Whether to use HTTPS or not"),
        )
        .arg(
            Arg::with_name("host")
                .long("host")
                .takes_value(true)
                .default_value("api.server.test")
                .help("Hostname to contact"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .takes_value(true)
                .default_value("8080")
                .help("Port to contact"),
        )
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!(
        "{}://{}:{}",
        if is_https { "https" } else { "http" },
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap()
    );

    let context: ClientContext = swagger::make_context!(
        ContextBuilder,
        EmptyContext,
        None as Option<AuthData>,
        XSpanIdString::default()
    );

    let mut client: Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client =
            Box::new(Client::try_new_https(&base_url).expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client =
            Box::new(Client::try_new_http(&base_url).expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        Some("QuestionsIdAnswersAnswerIdPut") => {
            let result = rt.block_on(client.questions_id_answers_answer_id_put(56, 56, None));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("QuestionsIdAnswersGet") => {
            let result = rt.block_on(client.questions_id_answers_get(56));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("QuestionsIdAnswersPatch") => {
            let result = rt.block_on(client.questions_id_answers_patch(56, Some(&Vec::new())));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("QuestionsIdAnswersPost") => {
            let result = rt.block_on(client.questions_id_answers_post(56, Some(&Vec::new())));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("QuestionsIdDelete") => {
            let result = rt.block_on(client.questions_id_delete(56));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("QuestionsIdGet") => {
            let result = rt.block_on(client.questions_id_get(56));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("QuestionsIdPut") => {
            let result = rt.block_on(client.questions_id_put(56, None));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("QuestionsGet") => {
            let result = rt.block_on(client.questions_get());
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("QuestionsPost") => {
            let result = rt.block_on(client.questions_post(None));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
