use crate::schema::resources;
use diesel::Associations;
use empty_macro::add_orm_field;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[add_orm_field]
#[derive(Associations)]
#[diesel(belongs_to(Resource))]
pub struct Resource {
    resource_id: i32,
    key: String,
    rtype: i32,
    name: String,
    desc: String,
    sort: i32,
    path: String,
    index: bool,
    menu: bool,
    icon: String,
}
#[derive(ToSchema, Deserialize, Serialize)]
pub struct Route {
    path: String,
    index: bool,
    menu: bool,
    icon: String,
}
#[derive(ToSchema, Deserialize, Serialize)]
pub struct Api {
    path: String,
}
#[derive(ToSchema, Deserialize, Serialize)]
pub enum Type {
    TypeRoute(Route),
    TypeFunction,
    TypeApi(Api),
}
