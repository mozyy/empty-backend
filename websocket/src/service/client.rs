use empty_utils::diesel::db;
use empty_utils::tonic::Resp;
use proto::pb;
use tonic::Request;

pub struct Service {
    db: db::DbPool,
}

impl Default for Service {
  fn default() -> Self {
      Self {
          db: db::DbPool::new("websocket_v2"),
      }
  }
}

#[async_trait::async_trait]
impl pb::websocket::client::client_service_server::ClientService for Service {
    async fn list_user(
        &self,
        request: Request<pb::websocket::client::ListUserRequest>,
    ) -> Resp<pb::websocket::client::ListUserResponse> {
        todo!()
    }
    async fn get_user(
        &self,
        request: Request<pb::websocket::client::GetUserRequest>,
    ) -> Resp<pb::websocket::client::GetUserResponse> {
        todo!()
    }
    async fn get_user_by_user_id(
        &self,
        request: Request<pb::websocket::client::GetUserByUserIdRequest>,
    ) -> Resp<pb::websocket::client::GetUserByUserIdResponse> {
        todo!()
    }
    async fn create_user(
        &self,
        request: Request<pb::websocket::client::CreateUserRequest>,
    ) -> Resp<pb::websocket::client::CreateUserResponse> {
        todo!()
    }
    async fn update_user(
        &self,
        request: Request<pb::websocket::client::UpdateUserRequest>,
    ) -> Resp<pb::websocket::client::UpdateUserResponse> {
        todo!()
    }
    async fn delete_user(
        &self,
        request: Request<pb::websocket::client::DeleteUserRequest>,
    ) -> Resp<pb::websocket::client::DeleteUserResponse> {
        todo!()
    }
}
