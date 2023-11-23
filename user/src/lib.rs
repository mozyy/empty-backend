use proto::pb::wx::{
    user::user_service_server::UserServiceServer, wx::wx_service_server::WxServiceServer,
};
use service::{user};

pub(crate) mod dao;
pub(crate) mod service;

pub(crate) const CLIENT_ID: &str = "f2e69885-951a-4538-b0c8-67385f0c1420";

pub fn get_service() -> UserServiceServer<user::Service>{
    UserServiceServer::new(user::Service::default())
}
