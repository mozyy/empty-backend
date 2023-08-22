use axum::extract::State;
use axum::response::IntoResponse;

use oxide_auth::endpoint::{OwnerConsent, Solicitation, WebResponse};
use oxide_auth::frontends::simple::endpoint::FnSolicitor;

use oxide_auth_async::endpoint::authorization::AuthorizationFlow;
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};

use super::state;

pub async fn authorize_get(
    State(state): State<state::State>,
    request: OAuthRequest,
) -> impl IntoResponse {
    AuthorizationFlow::prepare(
        state
            .endpoint_state
            .endpoint()
            .await
            .with_solicitor(FnSolicitor(
                |_: &mut OAuthRequest, solicitation: Solicitation| {
                    let pre_g = &solicitation.pre_grant();
                    let state = &solicitation.state();
                    log::debug!("PreGrant: {:?}, {:?}", pre_g, state);

                    let _client_id = &solicitation.pre_grant().client_id;

                    let mut response = OAuthResponse::default();
                    response
                        .redirect("http://www.com".parse().unwrap())
                        .unwrap();
                    // OwnerConsent::InProgress(response)
                    OwnerConsent::Authorized("abc".into())
                },
            )),
    )?
    .execute(request)
    .await
    .map_err(WebError::from)
}

// async fn token(State(state): State<state::State>, request: OAuthRequest) -> impl IntoResponse {
//     let r = AccessTokenFlow::prepare(state.endpoint_state.endpoint().await)?
//         .execute(request)
//         .await;

//         let rr = r.map_err(WebError::from);
//         rr
// }
