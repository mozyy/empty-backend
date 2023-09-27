use proto::pb::wx::{
    user::user_service_server::UserServiceServer, wx::wx_service_server::WxServiceServer,
};
use service::{user, wx};

pub(crate) mod dao;
pub(crate) mod service;

pub fn get_service() -> (
    WxServiceServer<wx::Service>,
    UserServiceServer<user::Service>,
) {
    let wx = WxServiceServer::new(wx::Service::default());
    let user = UserServiceServer::new(user::Service::default());
    (wx, user)
}
