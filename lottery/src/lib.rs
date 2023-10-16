use empty_utils::diesel::db;
use proto::pb::lottery::{
    favorite::favorite_service_server::FavoriteServiceServer,
    lottery::lottery_service_server::LotteryServiceServer,
    record::record_service_server::RecordServiceServer,
    template::template_service_server::TemplateServiceServer,
};
use service::{favorite, lottery, record, template};

pub(crate) mod dao;
pub(crate) mod service;

pub fn get_service() -> (
    LotteryServiceServer<lottery::Service>,
    RecordServiceServer<record::Service>,
    FavoriteServiceServer<favorite::Service>,
    TemplateServiceServer<template::Service>,
) {
    let db_lottery = db::DbPool::new("lottery_v2");
    let lottery = LotteryServiceServer::new(lottery::Service::new_by_db(db_lottery.clone()));
    let record = RecordServiceServer::new(record::Service::new_by_db(db_lottery.clone()));
    let favorite = FavoriteServiceServer::new(favorite::Service::new_by_db(db_lottery.clone()));
    let template = TemplateServiceServer::new(template::Service::new_by_db(db_lottery.clone()));
    (lottery, record, favorite, template)
}
