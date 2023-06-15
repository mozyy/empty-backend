use crate::{
    configs::{MINI_APPID, MINI_SECRET},
    pb::wx as pb,
};

impl pb::SnsJscode2sessionRequest {
    pub fn new(js_code: String) -> Self {
        Self {
            appid: MINI_APPID.to_string(),
            secret: MINI_SECRET.to_string(),
            js_code,
            grant_type: String::from("authorization_code"),
        }
    }
}
