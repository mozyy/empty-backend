use std::ops::Deref;

use empty_utils::errors::Error;

use crate::pb;

pub mod diesel;
pub mod endpoint;
pub mod grpc;
pub mod primitives;
pub mod solicitor;

#[derive(Clone)]
pub struct UserId(pb::oauth::ResourceResponse);

impl<T> TryFrom<&tonic::Request<T>> for UserId {
    type Error = Error;

    fn try_from(value: &tonic::Request<T>) -> Result<Self, Self::Error> {
        let user_id = value
            .extensions()
            .get::<UserId>()
            .ok_or_else(|| Error::StatusError(tonic::Status::unauthenticated("no auth")))?;
        Ok(user_id.to_owned())
    }
}

impl UserId {
    pub fn new(res: pb::oauth::ResourceResponse) -> Self{
        Self(res)
    }
}

impl Deref for UserId {
    type Target = pb::oauth::ResourceResponse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for UserId {
    fn to_string(&self) -> String {
        self.owner_id.clone()
    }
}