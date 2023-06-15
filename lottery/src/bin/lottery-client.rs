use std::env;

use lottery::{configs::ADDR, pb};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    empty_utils::init();
    let mut client: pb::lottery::lottery_service_client::LotteryServiceClient<
        tonic::transport::Channel,
    > = pb::lottery::lottery_service_client::LotteryServiceClient::connect(ADDR)
        .await
        .unwrap();
    let mut client_user = pb::user::user_service_client::UserServiceClient::connect(ADDR)
        .await
        .unwrap();
    let create = client_user
        .create(pb::user::CreateRequest {
            user: Some(pb::user::NewUser {
                openid: Uuid::new_v4().to_string(),
                unionid: Uuid::new_v4().to_string(),
                session_key: Uuid::new_v4().to_string(),
                name: String::from("yyue"),
                avatar: None,
                mobile: None,
            }),
        })
        .await
        .unwrap();
    dbg!(&create);

    let create = client
        .create(tonic::Request::new(pb::lottery::CreateRequest {
            lottery: Some(pb::lottery::NewLottery {
                title: String::from("title"),
                user_id: create.into_inner().user.unwrap().id,
                r#type: pb::lottery::Type::Percent.into(),
                items: vec![
                    pb::lottery::Item {
                        name: String::from("item name1"),
                        value: 1,
                    },
                    pb::lottery::Item {
                        name: String::from("item name2"),
                        value: 2,
                    },
                ],
                remark: true,
                remarks: vec![],
            }),
        }))
        .await
        .unwrap();
    dbg!(create);
    let blogs = client
        .list(tonic::Request::new(pb::lottery::ListRequest {
            lottery: None,
            paginate: None,
        }))
        .await
        .unwrap();
    log::info!("connect success");
    dbg!(blogs);
}
