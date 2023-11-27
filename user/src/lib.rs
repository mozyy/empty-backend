use proto::pb::wx::{
    user::user_service_server::UserServiceServer, wx::wx_service_server::WxServiceServer,
};
use service::user;

pub(crate) mod dao;
pub(crate) mod service;
