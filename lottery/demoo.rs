pub mod demo {
    use uuid::Uuid;
    pub struct User {
        id: Uuid,
        created_at: ::empty_utils::diesel::timestamp::Timestamp,
        updated_at: ::empty_utils::diesel::timestamp::Timestamp,
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
        use diesel::row::{Row, Field};
        use std::convert::TryInto;
        impl<
            __DB: diesel::backend::Backend,
            __ST0,
            __ST1,
            __ST2,
        > Queryable<(__ST0, __ST1, __ST2), __DB> for User
        where
            (
                Uuid,
                ::empty_utils::diesel::timestamp::Timestamp,
                ::empty_utils::diesel::timestamp::Timestamp,
            ): FromStaticSqlRow<(__ST0, __ST1, __ST2), __DB>,
        {
            type Row = (
                Uuid,
                ::empty_utils::diesel::timestamp::Timestamp,
                ::empty_utils::diesel::timestamp::Timestamp,
            );
            fn build(row: Self::Row) -> deserialize::Result<Self> {
                Ok(Self {
                    id: row.0.try_into()?,
                    created_at: row.1.try_into()?,
                    updated_at: row.2.try_into()?,
                })
            }
        }
    };
}
