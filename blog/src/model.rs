use diesel::{connection::DefaultLoadingMode, prelude::*};
use empty_utils::errors::{Error, Result};
use proto::utils::diesel::Paginate;
use proto::{pb, schema};

pub async fn query_list(
    conn: &mut PgConnection,
    request: pb::blog::blog::ListRequest,
) -> Result<(
    Vec<pb::blog::blog::Blog>,
    Option<pb::utils::paginate::Paginated>,
)> {
    let blogs = schema::blog::blogs::table
        .filter(schema::blog::blogs::id.is_not_null())
        .paginate(request.paginate)
        .load_and_paginated::<pb::blog::blog::Blog>(conn)?;
    Ok(blogs)
}
pub async fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::blog::blog::Blog> {
    let blog = schema::blog::blogs::dsl::blogs.find(id).first(conn)?;
    Ok(blog)
}
pub async fn insert(
    conn: &mut PgConnection,
    blog: pb::blog::blog::NewBlog,
) -> Result<pb::blog::blog::Blog> {
    let blog = diesel::insert_into(schema::blog::blogs::dsl::blogs)
        .values(blog)
        .get_result(conn)?;
    Ok(blog)
}
// TODO: patch
pub async fn update_by_id(
    conn: &mut PgConnection,
    id: i32,
    blog: pb::blog::blog::NewBlog,
) -> Result<pb::blog::blog::Blog> {
    let blog = diesel::update(schema::blog::blogs::dsl::blogs)
        .filter(schema::blog::blogs::dsl::id.eq(id))
        .set(blog)
        .get_result(conn)?;
    Ok(blog)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(schema::blog::blogs::dsl::blogs.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
