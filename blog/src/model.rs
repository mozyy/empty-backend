use chrono::NaiveDateTime;
use diesel::{connection::DefaultLoadingMode, prelude::*};
use empty_utils::{
    convert::naive_date_time_to_timestamp,
    errors::{ServiceError, ServiceResult},
};
use proto::blog;
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

impl From<Blog> for blog::Blog {
    fn from(value: Blog) -> Self {
        let Blog {
            id,
            title,
            image,
            summary,
            markdown,
            author,
            created_at,
            updated_at,
        } = value;
        Self {
            id,
            title,
            image,
            summary,
            markdown,
            author,
            created_at: Some(naive_date_time_to_timestamp(created_at)),
            updated_at: Some(naive_date_time_to_timestamp(updated_at)),
        }
    }
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

impl From<NewBlog> for blog::NewBlog {
    fn from(value: NewBlog) -> Self {
        let NewBlog {
            title,
            image,
            summary,
            markdown,
            author,
        } = value;
        Self {
            title,
            image,
            summary,
            markdown,
            author,
        }
    }
}
impl From<blog::NewBlog> for NewBlog {
    fn from(value: blog::NewBlog) -> Self {
        let blog::NewBlog {
            title,
            image,
            summary,
            markdown,
            author,
        } = value;
        Self {
            title,
            image,
            summary,
            markdown,
            author,
        }
    }
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
pub async fn insert(conn: &mut PgConnection, blog: NewBlog) -> ServiceResult<Blog> {
    let blog = diesel::insert_into(blogs::dsl::blogs)
        .values(blog)
        .get_result(conn)?;
    Ok(blog)
}
// TODO: patch
pub async fn update_by_id(conn: &mut PgConnection, id: i32, blog: NewBlog) -> ServiceResult<Blog> {
    let blog = diesel::update(blogs::dsl::blogs)
        .filter(blogs::dsl::id.eq(id))
        .set(blog)
        .get_result(conn)?;
    Ok(blog)
}
pub async fn delete_by_id(conn: &mut PgConnection, id: i32) -> ServiceResult {
    let value = diesel::delete(blogs::dsl::blogs.find(id)).execute(conn)?;
    if value == 0 {
        return Err(ServiceError::String(String::from("delete error")));
    }
    Ok(())
}
