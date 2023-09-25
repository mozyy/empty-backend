use chrono::Utc;
use diesel::prelude::*;
use empty_utils::errors::{Error, Result, ErrorConvert};
use proto::pb;

use uuid::Uuid;

use proto::{schema, utils::diesel::Paginate};

pub fn query_by_token(
    conn: &mut PgConnection,
    access_token: String,
) -> Result<(pb::auth::auth::User, pb::auth::auth::Token)> {
    let resource:pb::auth::auth::Resource = schema::auth::resources::table
        .filter(schema::auth::resources::access_token.eq(access_token))
        .first(conn)?;
      let user_id = resource.user_id.parse::<Uuid>().ok_or_invalid()?;
    let user: pb::auth::auth::User = schema::auth::users::table.find(user_id).first(conn)?;
    let until = resource.until.ok_or_loss()?;
    let token = pb::auth::auth::Token {
        access_token: resource.access_token,
        refresh_token: resource.refresh_token,
        scope: resource.scope,
        token_type: resource.token_type,
        expires_in:  (until.seconds - Utc::now().timestamp()) as i32,
    };
    Ok((user, token))
}
