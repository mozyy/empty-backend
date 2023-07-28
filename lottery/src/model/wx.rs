use std::env;

use crate::pb::wx as pb;

impl pb::SnsJscode2sessionRequest {
    pub fn new(js_code: String) -> Self {
        Self {
            appid: env::var("MINI_APPID").expect("MINI_APPID must be set"),
            secret: env::var("MINI_SECRET").expect("MINI_SECRET must be set"),
            js_code,
            grant_type: String::from("authorization_code"),
        }
    }
}
