use proto::pb::wx::wx::wx_service_server::WxServiceServer;
use service::wx;

pub(crate) mod dao;
pub(crate) mod service;

pub fn get_service() -> WxServiceServer<wx::Service> {
    let wx = WxServiceServer::new(wx::Service::default());
    wx
}
