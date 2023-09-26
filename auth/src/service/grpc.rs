use empty_utils::tonic::Resp;

use crate::dao;
use proto::pb;

use super::Service;

#[tonic::async_trait]
impl pb::auth::auth::auth_service_server::AuthService for Service {
    async fn authorize(
        &self,
        _request: tonic::Request<pb::auth::auth::AuthorizeRequest>,
    ) -> Resp<pb::auth::auth::AuthorizeResponse> {
        todo!()
    }
    async fn token(
        &self,
        _request: tonic::Request<pb::auth::auth::TokenRequest>,
    ) -> Resp<pb::auth::auth::TokenResponse> {
        todo!()
    }
    async fn resource(
        &self,
        request: tonic::Request<pb::auth::auth::ResourceRequest>,
    ) -> Resp<pb::auth::auth::ResourceResponse> {
        let pb::auth::auth::ResourceRequest {
            uri: _,
            access_token,
        } = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let (_user, _token) = dao::resource::query_by_token(&mut conn, access_token)?;
        todo!()
        // Ok(tonic::Response::new(pb::auth::auth::ResourceResponse{user: Some(user), token: Some(token)}))
    }
    async fn login(
        &self,
        request: tonic::Request<pb::auth::auth::LoginRequest>,
    ) -> Resp<pb::auth::auth::LoginResponse> {
        let user_id = request.into_inner().user_id;
        let mut conn = self.db.get_conn()?;
        let _user = dao::user::query_by_id(&mut conn, user_id)?;
        todo!()
    }
    async fn register(
        &self,
        _request: tonic::Request<pb::auth::auth::RegisterRequest>,
    ) -> Resp<pb::auth::auth::RegisterResponse> {
        let mut conn = self.db.get_conn()?;
        let _user = dao::user::insert(&mut conn)?;
        todo!()
    }
    async fn client_list(
        &self,
        _request: tonic::Request<pb::auth::auth::ClientListRequest>,
    ) -> Resp<pb::auth::auth::ClientListResponse> {
        todo!()
    }
    async fn client_create(
        &self,
        _request: tonic::Request<pb::auth::auth::ClientCreateRequest>,
    ) -> Resp<pb::auth::auth::ClientCreateResponse> {
        todo!()
    }
    async fn config_list(
        &self,
        _request: tonic::Request<pb::auth::auth::ConfigListRequest>,
    ) -> Resp<pb::auth::auth::ConfigListResponse> {
        todo!()
    }
    async fn config_create(
        &self,
        _request: tonic::Request<pb::auth::auth::ConfigCreateRequest>,
    ) -> Resp<pb::auth::auth::ConfigCreateResponse> {
        todo!()
    }
}
