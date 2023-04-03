pub mod cmd;
pub mod convert;
pub mod diesel;
pub mod errors;
pub mod log;

// init dotenv, log
pub fn init() {
    dotenvy::dotenv().ok();
    log::init();
}
