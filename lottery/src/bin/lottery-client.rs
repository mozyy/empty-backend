use std::env;

use lottery::pb;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    empty_utils::init();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| String::from("http://0.0.0.0:50051"));
    log::info!("connect:{base_url}");
    let mut client: pb::lottery::lottery_service_client::LotteryServiceClient<tonic::transport::Channel> = pb::lottery::lottery_service_client::LotteryServiceClient::connect(base_url.to_owned())
        .await
        .unwrap();
    let mut client_user = pb::user::user_service_client::UserServiceClient::connect(base_url.to_owned()).await.unwrap();
    let create = client_user.create(pb::user::CreateRequest{ user: Some(pb::user::NewUser{ 
        openid: Uuid::new_v4().to_string(), unionid: Uuid::new_v4().to_string(), session_key: Uuid::new_v4().to_string(), name: String::from("yyue"), avatar: None, mobile: None }) }).await.unwrap();
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
        .list(tonic::Request::new(pb::lottery::ListRequest { lottery:None, paginate: None }))
        .await
        .unwrap();
    log::info!("connect success");
    dbg!(blogs);
}
