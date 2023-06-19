use oxide_auth::endpoint::WebResponse;
use url::Url;

use super::error::OAuthError;

#[derive(Debug)]
pub enum OAuthResponse {
    OK,
    REDIRECT(Url),
}
impl Default for OAuthResponse {
    fn default() -> Self {
        todo!()
    }
}

impl WebResponse for OAuthResponse {
    type Error = OAuthError;

    fn ok(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn redirect(&mut self, url: Url) -> Result<(), Self::Error> {
        todo!()
    }

    fn client_error(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn unauthorized(&mut self, header_value: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn body_text(&mut self, text: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn body_json(&mut self, data: &str) -> Result<(), Self::Error> {
        todo!()
    }
}
