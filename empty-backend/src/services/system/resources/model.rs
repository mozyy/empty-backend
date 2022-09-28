use std::time::SystemTime;
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct Resource {
    id: i32,
    resource_id: i32,
    key: String,
    r#type: Type,
    index: bool,
    path: String,
    name: String,
    menu: bool,
    icon: String,
    desc: String,
    sort: i32,
    created_at: SystemTime,
    updated_at: SystemTime,
}
#[derive(ToSchema)]
pub struct Route {
    path: String,
    index: bool,
}
#[derive(ToSchema)]
pub enum Type {
    TypeRoute(Route),
    TypeFunction,
    TypeApi(String, bool),
}
