use diesel::{connection::DefaultLoadingMode, prelude::*};
use empty_utils::errors::{Error, Result};

use crate::{pb, schema::blogs};

pub async fn query_list(conn: &mut PgConnection) -> Result<Vec<pb::Blog>> {
    let blogs = blogs::dsl::blogs
        .load_iter::<pb::Blog, DefaultLoadingMode>(conn)?
        .collect::<QueryResult<Vec<pb::Blog>>>()?;
    Ok(blogs)
}
pub async fn query_by_id(conn: &mut PgConnection, id: i32) -> Result<pb::Blog> {
    let blog = blogs::dsl::blogs.find(id).first(conn)?;
    Ok(blog)
}
pub async fn insert(conn: &mut PgConnection, blog: pb::NewBlog) -> Result<pb::Blog> {
    let blog = diesel::insert_into(blogs::dsl::blogs)
        .values(blog)
        .get_result(conn)?;
    Ok(blog)
}
// TODO: patch
pub async fn update_by_id(conn: &mut PgConnection, id: i32, blog: pb::NewBlog) -> Result<pb::Blog> {
    let blog = diesel::update(blogs::dsl::blogs)
        .filter(blogs::dsl::id.eq(id))
        .set(blog)
        .get_result(conn)?;
    Ok(blog)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result {
    let value = diesel::delete(blogs::dsl::blogs.find(id)).execute(conn)?;
    if value == 0 {
        return Err(Error::String(String::from("delete error")));
    }
    Ok(())
}
