use empty_utils::{errors::Error, tonic::Resp};
use proto::pb::file::{self, file_service_server::FileService};
use tonic::Request;

#[derive(Default)]
pub struct Service {}

#[tonic::async_trait]
impl FileService for Service {
    async fn list(&self, _: Request<file::ListRequest>) -> Resp<file::ListResponse> {
        todo!()
    }
    async fn get(&self, _: Request<file::GetRequest>) -> Resp<file::GetResponse> {
        todo!()
    }
    async fn create(&self, request: Request<file::CreateRequest>) -> Resp<file::CreateResponse> {
        let request = request.into_inner();
        let _file = request
            .file
            .ok_or_else(|| Error::StatusError(tonic::Status::invalid_argument("no file")))?;

        todo!()
    }
    async fn update(&self, _: Request<file::UpdateRequest>) -> Resp<file::UpdateResponse> {
        todo!()
    }
    async fn delete(&self, _: Request<file::DeleteRequest>) -> Resp<file::DeleteResponse> {
        todo!()
    }
}
