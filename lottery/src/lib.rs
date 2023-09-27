use empty_utils::diesel::db;
use proto::pb::lottery::{
    lottery::lottery_service_server::LotteryServiceServer,
    record::record_service_server::RecordServiceServer,
};
use service::{lottery, record};

pub(crate) mod dao;
pub(crate) mod service;

pub fn get_service() -> (
    LotteryServiceServer<lottery::Service>,
    RecordServiceServer<record::Service>,
) {
    let db_lottery = db::DbPool::new("lottery_v2");
    let lottery = LotteryServiceServer::new(lottery::Service::new_by_db(db_lottery.clone()));
    let record = RecordServiceServer::new(record::Service::new_by_db(db_lottery));
    (lottery, record)
}
