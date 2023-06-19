pub struct OAuthError(pub tonic::Status);

impl From<oxide_auth::endpoint::OAuthError> for OAuthError {
    fn from(value: oxide_auth::endpoint::OAuthError) -> Self {
        Self(tonic::Status::unauthenticated(value.to_string()))
    }
}
