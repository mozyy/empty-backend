use crate::schema::users;
use uuid::Uuid;

#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable)]
pub struct User {
    id: Uuid,
    created_at: ::empty_utils::diesel::timestamp::Timestamp,
    updated_at: ::empty_utils::diesel::timestamp::Timestamp,
}
