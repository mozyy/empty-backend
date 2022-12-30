use crate::errors::ServiceError;

pub mod oauth;
pub mod questions;

type Result<T> = std::result::Result<T, ServiceError>;
