use empty_utils::{diesel::db, errors::Result};
use proto::pb::user::{
    auth::auth_service_server::AuthServiceServer, user::user_service_server::UserServiceServer,
};
use service::{auth, check, user};

pub(crate) mod dao;
pub(crate) mod model;
pub(crate) mod service;
pub(crate) mod util;

pub async fn get_service() -> Result<(
    check::Service,
    AuthServiceServer<auth::Service>,
    UserServiceServer<user::Service>,
)> {
    let db_user = db::DbPool::new("user_v2");
    let check = check::Service::new_by_db(db_user.clone()).await?;
    let auth = AuthServiceServer::new(auth::Service::new_by_db(db_user.clone()));
    let user = UserServiceServer::new(user::Service::new_by_db(db_user));
    Ok((check, auth, user))
}
