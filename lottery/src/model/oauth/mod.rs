use std::ops::Deref;

use empty_utils::{errors::ServiceError, tonic::Resp};

pub mod diesel;
pub mod endpoint;
pub mod grpc;
pub mod primitives;
pub mod solicitor;

#[derive(Clone)]
pub struct UserId(pub String);

impl<T> TryFrom<&tonic::Request<T>> for UserId {
    type Error = ServiceError;

    fn try_from(value: &tonic::Request<T>) -> Result<Self, Self::Error> {
        let user_id = value
            .extensions()
            .get::<UserId>()
            .ok_or_else(|| ServiceError::StatusError(tonic::Status::unauthenticated("no auth")))?;
        Ok(user_id.to_owned())
    }
}

impl Deref for UserId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
