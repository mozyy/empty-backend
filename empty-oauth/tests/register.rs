use empty_oauth::pb::oauth_server::OauthServer;
use empty_oauth::pb::{oauth_client::OauthClient, OauthRequest};
use empty_oauth::Service;
use std::{future::Future, sync::Arc};
use tempfile::NamedTempFile;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint, Server, Uri};
use tower::service_fn;

// The actual test is here
#[tokio::test]
async fn add_merchant_test() {
    let (serve_future, mut client) = server_and_client_stub().await;

    let request_future = async {
        let response = client
            .oauth(OauthRequest {
                name: "world".to_string(),
            })
            .await
            .unwrap()
            .into_inner();
        // Validate server response with assertions
        assert_eq!(
            response.response,
            "response oauth service: world".to_string()
        );
    };

    // Wait for completion, when the client request future completes
    tokio::select! {
        _ = serve_future => panic!("server returned first"),
        _ = request_future => (),
    }
}

async fn server_and_client_stub() -> (impl Future<Output = ()>, OauthClient<Channel>) {
    let socket = NamedTempFile::new().unwrap();
    let socket = Arc::new(socket.into_temp_path());
    std::fs::remove_file(&*socket).unwrap();

    let uds = UnixListener::bind(&*socket).unwrap();
    let stream = UnixListenerStream::new(uds);

    let serve_future = async {
        let result = Server::builder()
            .add_service(OauthServer::new(Service::default()))
            .serve_with_incoming(stream)
            .await;
        // Server must be running fine...
        assert!(result.is_ok());
    };

    let socket = Arc::clone(&socket);
    // Connect to the server over a Unix socket
    // The URL will be ignored.
    let channel = Endpoint::try_from("http://any.url")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let socket = Arc::clone(&socket);
            async move { UnixStream::connect(&*socket).await }
        }))
        .await
        .unwrap();

    let client = OauthClient::new(channel);

    (serve_future, client)
}
