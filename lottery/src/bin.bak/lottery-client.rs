use empty_utils::errors::{Error, Result};
use lottery::configs::ADDR_CLIENT;
use proto::pb;

#[tokio::main]
async fn main() -> Result {
    empty_utils::init();
    let mut client =
        pb::oauth::oauth::o_auth_service_client::OAuthServiceClient::connect(ADDR_CLIENT)
            .await
            .map_err(Error::other)?;
    // client
    //     .client_create(pb::oauth::oauth::ClientCreateRequest {
    //         client: Some(pb::oauth::oauth::NewClient {
    //             name: "zuoyinyun".into(),
    //             default_scope: "logined".into(),
    //             redirect_uri: "https://zuoyinyun.com".into(),
    //             passdata: None,
    //         }),
    //     })
    //     .await
    //     .unwrap();
    let res = client
        .config_create(pb::oauth::oauth::ConfigCreateRequest {
            config: Some(pb::oauth::oauth::NewConfig {
                pattern: Some(pb::oauth::oauth::Pattern {
                    pattern: Some(pb::oauth::oauth::pattern::Pattern::Regex(".*".into())),
                }),
                scope: None,
            }),
        })
        .await
        .map_err(Error::other)?;
    dbg!(res);
    // let mut client =
    //     pb::oauth::lottery::lottery_service_client::LotteryServiceClient::connect(ADDR_CLIENT)
    //         .await
    //         .unwrap();
    // let mut client_user = pb::oauth::user::user_service_client::UserServiceClient::connect(ADDR_CLIENT)
    //     .await
    //     .unwrap();
    // let create = client_user
    //     .create(pb::oauth::user::CreateRequest {
    //         wx_user: Some(pb::oauth::user::NewWxUser {
    //             user_id: Uuid::new_v4().to_string(),
    //             openid: Uuid::new_v4().to_string(),
    //             unionid: Some(Uuid::new_v4().to_string()),
    //             session_key: Uuid::new_v4().to_string(),
    //             name: String::from("yyue"),
    //             avatar: None,
    //             mobile: None,
    //         }),
    //     })
    //     .await
    //     .unwrap();
    // dbg!(&create);

    // let create = client
    //     .create(tonic::Request::new(pb::oauth::lottery::CreateRequest {
    //         lottery: Some(pb::oauth::lottery::NewLottery {
    //             title: String::from("title"),
    //             user_id: create.into_inner().wx_user.unwrap().id,
    //             r#type: pb::oauth::lottery::Type::Percent.into(),
    //             items: vec![
    //                 pb::oauth::lottery::Item {
    //                     name: String::from("item name1"),
    //                     value: 1,
    //                 },
    //                 pb::oauth::lottery::Item {
    //                     name: String::from("item name2"),
    //                     value: 2,
    //                 },
    //             ],
    //             remark: true,
    //             remarks: vec![],
    //         }),
    //     }))
    //     .await
    //     .unwrap();
    // dbg!(create);
    // let mut req = tonic::Request::new(pb::oauth::lottery::ListRequest {
    //     lottery: None,
    //     paginate: None,
    // });
    // req.metadata_mut()
    //     .append("authorization", "Bearer aaaaa".parse().unwrap());
    // let blogs = client.list(req).await.unwrap();
    // log::info!("connect success");
    // dbg!(blogs);
    Ok(())
}
