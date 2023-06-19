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

    fn redirect(&mut self, _url: Url) -> Result<(), Self::Error> {
        todo!()
    }

    fn client_error(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn unauthorized(&mut self, _header_value: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn body_text(&mut self, _text: &str) -> Result<(), Self::Error> {
        todo!()
    }

    fn body_json(&mut self, _data: &str) -> Result<(), Self::Error> {
        todo!()
    }
}
