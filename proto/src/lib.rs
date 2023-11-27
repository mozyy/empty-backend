use empty_utils::errors::Error;
use std::ops::Deref;

pub mod pb;

pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
}

pub mod model;
pub mod schema;
pub mod types;
pub mod utils;
