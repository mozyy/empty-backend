use empty_utils::{diesel::db, errors::ServiceResult};
use oxide_auth::{
    frontends::simple::endpoint::{Generic, Vacant},
    primitives::prelude::{AuthMap, ClientMap, RandomGenerator, Scope, TokenMap},
};
use oxide_auth_axum::OAuthResponse;

use crate::model;

pub type ServiceEndpoint = Generic<
    ClientMap,
    AuthMap<RandomGenerator>,
    TokenMap<RandomGenerator>,
    Vacant,
    Vec<Scope>,
    fn() -> OAuthResponse,
>;

#[derive(Default, Clone)]
pub struct Service {
    db: db::DbPool,
}

impl Service {
    pub async fn get(&mut self) -> ServiceResult<Vec<model::Blog>> {
        let mut conn = self.db.get_conn()?;
        let blogs = model::query_list(&mut conn).await?;
        Ok(blogs)
    }
    pub async fn get_item(&mut self, id: i32) -> ServiceResult<model::Blog> {
        let mut conn = self.db.get_conn()?;
        let blogs = model::query_by_id(&mut conn, id).await?;
        Ok(blogs)
    }
    pub async fn post_item(&mut self, blog: model::NewBlog) -> ServiceResult<i32> {
        let mut conn = self.db.get_conn()?;
        let i32 = model::insert(&mut conn, blog).await?;
        Ok(i32)
    }
    pub async fn put_item(&mut self, id: i32, blog: model::NewBlog) -> ServiceResult {
        let mut conn = self.db.get_conn()?;
        model::update_by_id(&mut conn, id, blog).await?;
        Ok(())
    }
    pub async fn delete_item(&mut self, id: i32) -> ServiceResult {
        let mut conn = self.db.get_conn()?;
        model::delete_by_id(&mut conn, id).await?;
        Ok(())
    }
}
