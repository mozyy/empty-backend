pub mod cmd;
pub mod convert;
pub mod diesel;
pub mod log;
pub mod tonic;

// init dotenv, log
pub fn init() {
    dotenvy::dotenv().ok();
    log::init();
}
