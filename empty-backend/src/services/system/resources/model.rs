use diesel::sql_types::Timestamp;
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct Resource {
    id: i32,
    created_at: Timestamp,
    updated_at: Timestamp,
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
