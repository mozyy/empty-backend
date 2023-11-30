use empty_utils::{
    errors::Error,
    tonic::{Resp, ToResp},
};

use crate::{dao, model};
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
        let pb::auth::auth::ResourceRequest { uri, access_token } = request.into_inner();
        let scope_uri = self.get_scope_by_uri(uri).await;
        let resource = self.get_resource_by_access_token(&access_token).await?;
        let scope_token = resource
            .scope
            .parse::<model::config::Scope>()
            .unwrap_or_default();
        if scope_token < scope_uri {
            Err(Error::StatusError(tonic::Status::permission_denied(
                format!("token:{scope_token},uri:{scope_uri}"),
            )))?
        } else {
            pb::auth::auth::ResourceResponse {
                owner_id: resource.user_id,
                client_id: resource.client_id,
                scope: resource.scope,
                until: resource.until,
            }
            .to_resp()
        }
    }
    async fn login(
        &self,
        request: tonic::Request<pb::auth::auth::LoginRequest>,
    ) -> Resp<pb::auth::auth::LoginResponse> {
        let pb::auth::auth::LoginRequest { user_id, client_id } = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let user = dao::user::query_by_id(&mut conn, user_id)?;
        let client = dao::client::query_by_id(&mut conn, client_id)?;
        let resource = pb::auth::auth::NewResource::generate(&user, &client);
        let resource = dao::resource::insert(&mut conn, resource)?;
        self.refresh_resources().await?;
        pb::auth::auth::LoginResponse {
            user: Some(user),
            token: Some((&resource).into()),
        }
        .to_resp()
    }
    async fn register(
        &self,
        request: tonic::Request<pb::auth::auth::RegisterRequest>,
    ) -> Resp<pb::auth::auth::RegisterResponse> {
        let client_id = request.into_inner().client_id;
        let mut conn = self.db.get_conn()?;
        let user = dao::user::insert(&mut conn)?;
        let client = dao::client::query_by_id(&mut conn, client_id)?;
        let resource = pb::auth::auth::NewResource::generate(&user, &client);
        let resource = dao::resource::insert(&mut conn, resource)?;
        self.refresh_resources().await?;
        pb::auth::auth::RegisterResponse {
            user: Some(user),
            token: Some((&resource).into()),
        }
        .to_resp()
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
