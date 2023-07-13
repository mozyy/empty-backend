use lottery::{configs::ADDR_CLIENT, pb};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    empty_utils::init();
    let mut client = pb::oauth::o_auth_service_client::OAuthServiceClient::connect(ADDR_CLIENT)
        .await
        .unwrap();
    // client
    //     .client_create(pb::oauth::ClientCreateRequest {
    //         client: Some(pb::oauth::NewClient {
    //             name: "zuoyinyun".into(),
    //             default_scope: "logined".into(),
    //             redirect_uri: "https://zuoyinyun.com".into(),
    //             passdata: None,
    //         }),
    //     })
    //     .await
    //     .unwrap();
    let res = client
        .config_create(pb::oauth::ConfigCreateRequest {
            config: Some(pb::oauth::NewConfig {
                pattern: Some(pb::oauth::Pattern {
                    pattern: Some(pb::oauth::pattern::Pattern::Regex(
                        ".*".into(),
                    )),
                }),
                scope: None,
            }),
        })
        .await
        .unwrap();
    dbg!(res);
    // let mut client =
    //     pb::lottery::lottery_service_client::LotteryServiceClient::connect(ADDR_CLIENT)
    //         .await
    //         .unwrap();
    // let mut client_user = pb::user::user_service_client::UserServiceClient::connect(ADDR_CLIENT)
    //     .await
    //     .unwrap();
    // let create = client_user
    //     .create(pb::user::CreateRequest {
    //         wx_user: Some(pb::user::NewWxUser {
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
    //     .create(tonic::Request::new(pb::lottery::CreateRequest {
    //         lottery: Some(pb::lottery::NewLottery {
    //             title: String::from("title"),
    //             user_id: create.into_inner().wx_user.unwrap().id,
    //             r#type: pb::lottery::Type::Percent.into(),
    //             items: vec![
    //                 pb::lottery::Item {
    //                     name: String::from("item name1"),
    //                     value: 1,
    //                 },
    //                 pb::lottery::Item {
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
    // let mut req = tonic::Request::new(pb::lottery::ListRequest {
    //     lottery: None,
    //     paginate: None,
    // });
    // req.metadata_mut()
    //     .append("authorization", "Bearer aaaaa".parse().unwrap());
    // let blogs = client.list(req).await.unwrap();
    // log::info!("connect success");
    // dbg!(blogs);
}
