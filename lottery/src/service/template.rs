use empty_utils::{
    diesel::db,
    errors::ErrorConvert,
    tonic::{Resp, ToResp},
};
use tonic::{Request, Response};

use crate::dao;
use proto::pb;

pub struct Service {
    db: db::DbPool,
}

impl Service {
    pub fn new_by_db(db: db::DbPool) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl pb::lottery::template::template_service_server::TemplateService for Service {
    async fn list(
        &self,
        request: Request<pb::lottery::template::ListRequest>,
    ) -> Resp<pb::lottery::template::ListResponse> {
        let request = request.into_inner();
        let mut conn = self.db.get_conn()?;
        let (templates, paginated) = dao::template::query_list(&mut conn, request)?;
        pb::lottery::template::ListResponse {
            templates,
            paginated,
        }
        .to_resp()
    }
    async fn get(
        &self,
        request: Request<pb::lottery::template::GetRequest>,
    ) -> Resp<pb::lottery::template::GetResponse> {
        let id = request.into_inner().id;
        let mut conn = self.db.get_conn()?;
        let template = dao::template::query_by_id(&mut conn, id)?;
        pb::lottery::template::GetResponse {
            template: Some(template),
        }
        .to_resp()
    }
    async fn get_by_lottery_id(
        &self,
        request: Request<pb::lottery::template::GetByLotteryIdRequest>,
    ) -> Resp<pb::lottery::template::GetByLotteryIdResponse> {
        let lottery_id = request.into_inner().lottery_id;
        let mut conn = self.db.get_conn()?;
        let template = dao::template::query_by_lottery_id(&mut conn, lottery_id).ok();
        pb::lottery::template::GetByLotteryIdResponse {
            template,
        }
        .to_resp()
    }
    async fn create(
        &self,
        request: Request<pb::lottery::template::CreateRequest>,
    ) -> Resp<pb::lottery::template::CreateResponse> {
        let template = request.into_inner().template.ok_or_loss()?;
        let mut conn = self.db.get_conn()?;
        let template = dao::template::insert(&mut conn, template)?;
        pb::lottery::template::CreateResponse {
            template: Some(template),
        }
        .to_resp()
    }
    async fn update(
        &self,
        request: Request<pb::lottery::template::UpdateRequest>,
    ) -> Resp<pb::lottery::template::UpdateResponse> {
        let pb::lottery::template::UpdateRequest { id, template } = request.into_inner();
        let template = template.ok_or_loss()?;
        let mut conn = self.db.get_conn()?;
        let template = dao::template::update_by_id(&mut conn, id, template)?;
        pb::lottery::template::UpdateResponse {
            template: Some(template),
        }
        .to_resp()
    }
    async fn delete(
        &self,
        request: Request<pb::lottery::template::DeleteRequest>,
    ) -> Resp<pb::lottery::template::DeleteResponse> {
        let id = request.into_inner().id;
        let mut conn = self.db.get_conn()?;
        dao::template::delete_by_id(&mut conn, id)?;
        pb::lottery::template::DeleteResponse {}.to_resp()
    }
}
