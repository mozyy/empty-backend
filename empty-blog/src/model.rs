use chrono::NaiveDateTime;
use diesel::{connection::DefaultLoadingMode, prelude::*};
use empty_utils::errors::{ServiceError, ServiceResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::blogs;

#[derive(Queryable, Serialize, ToSchema)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub summary: String,
    pub markdown: String,
    pub author: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Insertable, AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name =blogs)]
pub struct NewBlog {
    pub title: String,
    pub image: String,
    pub summary: String,
    pub markdown: String,
    pub author: String,
}

pub async fn query_list(conn: &mut PgConnection) -> ServiceResult<Vec<Blog>> {
    let blogs = blogs::dsl::blogs
        .load_iter::<Blog, DefaultLoadingMode>(conn)?
        .collect::<QueryResult<Vec<Blog>>>()?;
    Ok(blogs)
}
pub async fn query_by_id(conn: &mut PgConnection, id: i32) -> ServiceResult<Blog> {
    let blog = blogs::dsl::blogs.find(id).first(conn)?;
    Ok(blog)
}
pub async fn insert(conn: &mut PgConnection, blog: NewBlog) -> ServiceResult<i32> {
    let id = diesel::insert_into(blogs::dsl::blogs)
        .values(blog)
        .returning(blogs::id)
        .get_result(conn)?;
    Ok(id)
}
pub async fn update_by_id(conn: &mut PgConnection, id: i32, blog: NewBlog) -> ServiceResult {
    diesel::update(blogs::dsl::blogs)
        .filter(blogs::dsl::id.eq(id))
        .set(blog)
        .execute(conn)?;
    Ok(())
}
pub async fn delete_by_id(conn: &mut PgConnection, id: i32) -> ServiceResult {
    let value = diesel::delete(blogs::dsl::blogs.find(id)).execute(conn)?;
    if value == 0 {
        return Err(ServiceError::String(String::from("delete error")));
    }
    Ok(())
}
