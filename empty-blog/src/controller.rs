use axum::{
    extract::{Path, Query, State},
    Json,
};
use empty_utils::errors::ServiceResult;

use crate::{
    model::{self, Blog, NewBlog},
    service::Service,
};

#[utoipa::path(
    get,
    path = "/",
    responses((status = 200, body = [Blog]))
)]
pub async fn get(State(mut state): State<Service>) -> ServiceResult<Json<Vec<Blog>>> {
    log::info!("receive get request");
    let blogs = state.get().await?;
    Ok(Json(blogs))
}

#[utoipa::path(
    get,
    path = "/{id}",
    responses((status = 200, body = Blog))
)]
pub async fn get_item(
    State(mut state): State<Service>,
    Path(id): Path<i32>,
) -> ServiceResult<Json<Blog>> {
    let blog = state.get_item(id).await?;
    Ok(Json(blog))
}

#[utoipa::path(
    post,
    path = "/",
    request_body=NewBlog,
    responses((status = 200, body = [Blog]))
)]
pub async fn post_item(
    State(mut state): State<Service>,
    Json(blog): Json<NewBlog>,
) -> ServiceResult<String> {
    Ok(state.post_item(blog).await?.to_string())
}

#[utoipa::path(
    put,
    path = "/{id}",
    request_body=NewBlog,
    responses((status = 200))
)]
pub async fn put_item(
    State(mut state): State<Service>,
    Path(id): Path<i32>,
    Json(blog): Json<NewBlog>,
) -> ServiceResult {
    state.put_item(id, blog).await
}

#[utoipa::path(
    delete,
    path = "/{id}",
    responses((status = 200))
)]
pub async fn delete_item(State(mut state): State<Service>, Path(id): Path<i32>) -> ServiceResult {
    state.delete_item(id).await
}
