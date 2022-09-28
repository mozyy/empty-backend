mod demo {
    use crate::schema::questions;
    extern crate empty_utils;
    use empty_utils::add_orm_field;
    #[diesel(table_name = questions)]
    pub struct Question {
        pub id: i32,
        pub content: String,
        pub desc: Option<String>,
        #[serde(with = "crate::utils::timestamp")]
        #[schema(value_type = i64)]
        pub created_at: chrono::NaiveDateTime,
        #[serde(with = "crate::utils::timestamp")]
        #[schema(value_type = i64)]
        pub updated_at: chrono::NaiveDateTime,
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
            __ST3,
            __ST4,
        > Queryable<(__ST0, __ST1, __ST2, __ST3, __ST4), __DB> for Question
        where
            (
                i32,
                String,
                Option<String>,
                chrono::NaiveDateTime,
                chrono::NaiveDateTime,
            ): FromStaticSqlRow<(__ST0, __ST1, __ST2, __ST3, __ST4), __DB>,
        {
            type Row = (
                i32,
                String,
                Option<String>,
                chrono::NaiveDateTime,
                chrono::NaiveDateTime,
            );
            fn build(row: Self::Row) -> deserialize::Result<Self> {
                Ok(Self {
                    id: row.0.try_into()?,
                    content: row.1.try_into()?,
                    desc: row.2.try_into()?,
                    created_at: row.3.try_into()?,
                    updated_at: row.4.try_into()?,
                })
            }
        }
    };
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for Question {
            type Table = questions::table;
            fn table() -> Self::Table {
                questions::table
            }
        }
        impl<'ident> Identifiable for &'ident Question {
            type Id = (&'ident i32);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
        impl<'ident> Identifiable for &'_ &'ident Question {
            type Id = (&'ident i32);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Question {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Question",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "content",
                    &self.content,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "desc",
                    &self.desc,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "created_at",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a chrono::NaiveDateTime,),
                            phantom: _serde::__private::PhantomData<Question>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                crate::utils::timestamp::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.created_at,),
                            phantom: _serde::__private::PhantomData::<Question>,
                        }
                    },
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "updated_at",
                    {
                        struct __SerializeWith<'__a> {
                            values: (&'__a chrono::NaiveDateTime,),
                            phantom: _serde::__private::PhantomData<Question>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                crate::utils::timestamp::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.updated_at,),
                            phantom: _serde::__private::PhantomData::<Question>,
                        }
                    },
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl utoipa::ToSchema for Question {
        fn schema() -> utoipa::openapi::schema::Schema {
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    "id",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::Integer)
                        .format(Some(utoipa::openapi::SchemaFormat::Int32)),
                )
                .required("id")
                .property(
                    "content",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::String),
                )
                .required("content")
                .property(
                    "desc",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::String),
                )
                .property(
                    "created_at",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::Integer)
                        .format(Some(utoipa::openapi::SchemaFormat::Int64)),
                )
                .required("created_at")
                .property(
                    "updated_at",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::Integer)
                        .format(Some(utoipa::openapi::SchemaFormat::Int64)),
                )
                .required("updated_at")
                .into()
        }
    }
    #[diesel(table_name = questions)]
    pub struct NewQuestion {
        pub content: String,
        pub desc: Option<String>,
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::internal::derives::insertable::UndecoratedInsertRecord;
        use diesel::prelude::*;
        impl Insertable<questions::table> for NewQuestion {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<questions::content, String>>,
                std::option::Option<diesel::dsl::Eq<questions::desc, String>>,
            ) as Insertable<questions::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<questions::content, String>>,
                std::option::Option<diesel::dsl::Eq<questions::desc, String>>,
            ) as Insertable<questions::table>>::Values {
                (
                    std::option::Option::Some(questions::content.eq(self.content)),
                    self.desc.map(|x| questions::desc.eq(x)),
                )
                    .values()
            }
        }
        impl<'insert> Insertable<questions::table> for &'insert NewQuestion {
            type Values = <(
                std::option::Option<
                    diesel::dsl::Eq<questions::content, &'insert String>,
                >,
                std::option::Option<diesel::dsl::Eq<questions::desc, &'insert String>>,
            ) as Insertable<questions::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<
                    diesel::dsl::Eq<questions::content, &'insert String>,
                >,
                std::option::Option<diesel::dsl::Eq<questions::desc, &'insert String>>,
            ) as Insertable<questions::table>>::Values {
                (
                    std::option::Option::Some(questions::content.eq(&self.content)),
                    self.desc.as_ref().map(|x| questions::desc.eq(x)),
                )
                    .values()
            }
        }
        impl UndecoratedInsertRecord<questions::table> for NewQuestion {}
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for NewQuestion {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "content" => _serde::__private::Ok(__Field::__field0),
                            "desc" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"content" => _serde::__private::Ok(__Field::__field0),
                            b"desc" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<NewQuestion>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = NewQuestion;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct NewQuestion",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct NewQuestion with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct NewQuestion with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(NewQuestion {
                            content: __field0,
                            desc: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "content",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            String,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("desc"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("content") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("desc") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(NewQuestion {
                            content: __field0,
                            desc: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["content", "desc"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "NewQuestion",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<NewQuestion>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl utoipa::ToSchema for NewQuestion {
        fn schema() -> utoipa::openapi::schema::Schema {
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    "content",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::String),
                )
                .required("content")
                .property(
                    "desc",
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::SchemaType::String),
                )
                .into()
        }
    }
}
