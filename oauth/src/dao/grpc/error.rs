use empty_utils::errors::Error;

#[derive(Debug)]
pub struct OAuthError(pub tonic::Status);

impl From<oxide_auth::endpoint::OAuthError> for OAuthError {
    fn from(value: oxide_auth::endpoint::OAuthError) -> Self {
        Self(tonic::Status::unauthenticated(value.to_string()))
    }
}

impl From<OAuthError> for Error {
    fn from(value: OAuthError) -> Self {
        Self::StatusError(value.0)
    }
}

impl From<OAuthError> for tonic::Status {
    fn from(value: OAuthError) -> Self {
        value.0
    }
}
