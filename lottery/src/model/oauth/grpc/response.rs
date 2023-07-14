use empty_utils::tonic::Resp;
use http::header;
use oxide_auth::endpoint::WebResponse;
use url::Url;

use super::error::OAuthError;

#[derive(Debug)]
pub enum ResponseStatus {
    Ok,
    Redirect(Url),
    BadRequest,
    Unauthorized(String),
}
#[derive(Debug)]
pub struct OAuthResponse {
    pub status: ResponseStatus,
    pub body: Option<String>,
}
impl Default for OAuthResponse {
    fn default() -> Self {
        Self {
            status: ResponseStatus::Ok,
            body: None,
        }
    }
}

impl WebResponse for OAuthResponse {
    type Error = OAuthError;

    fn ok(&mut self) -> Result<(), Self::Error> {
        self.status = ResponseStatus::Ok;
        Ok(())
    }

    fn redirect(&mut self, url: Url) -> Result<(), Self::Error> {
        self.status = ResponseStatus::Redirect(url);
        Ok(())
    }

    fn client_error(&mut self) -> Result<(), Self::Error> {
        self.status = ResponseStatus::BadRequest;
        Ok(())
    }

    fn unauthorized(&mut self, header_value: &str) -> Result<(), Self::Error> {
        self.status = ResponseStatus::Unauthorized(header_value.to_string());
        Ok(())
    }

    fn body_text(&mut self, text: &str) -> Result<(), Self::Error> {
        self.body = Some(text.to_string());
        Ok(())
    }

    fn body_json(&mut self, data: &str) -> Result<(), Self::Error> {
        self.body = Some(data.to_string());
        Ok(())
    }
}

impl<T> From<OAuthResponse> for Resp<T>
where
    T: Default,
{
    fn from(value: OAuthResponse) -> Self {
        match value.status {
            ResponseStatus::Redirect(url) => {
                let mut headers = http::HeaderMap::new();
                headers.insert(header::LOCATION, url.to_string().parse().unwrap());
                let meta = tonic::metadata::MetadataMap::from_headers(headers);
                let status = tonic::Status::with_metadata(tonic::Code::Ok, "redirect", meta);
                Err(status)
            }
            ResponseStatus::BadRequest => Err(tonic::Status::unknown("BAD_REQUEST")),
            ResponseStatus::Unauthorized(e) => Err(tonic::Status::unauthenticated(e)),
            ResponseStatus::Ok => Ok(tonic::Response::new(Default::default())),
        }
    }
}
