fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
    // lottery
    .type_attribute(
        "lottery.Item",
        "#[derive(::diesel::FromSqlRow, ::diesel::AsExpression)]
        #[diesel(sql_type = crate::schema::sql_types::Item)]",
    )
    .type_attribute(
        "lottery.Remark",
        "#[derive(::diesel::FromSqlRow, ::diesel::AsExpression)]
        #[diesel(sql_type = crate::schema::sql_types::Remark)]",
    )
    .type_attribute(
        "lottery.Lottery",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lotterys, belongs_to(crate::pb::user::User))]",
    )
    .field_attribute(
        "lottery.Lottery.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "lottery.NewLottery",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lotterys, belongs_to(crate::pb::user::User))]",
    )
    .field_attribute(
        "lottery.NewLottery.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid, serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "lottery.Lottery.type",
        "#[diesel(column_name = \"type_\")]",
    )
    .field_attribute(
        "lottery.NewLottery.type",
        "#[diesel(column_name = \"type_\")]",
    )

    // oauth
    .type_attribute(
        "oauth.User",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable)]
    #[diesel(table_name=crate::schema::oauth_users)]",
    )
    .type_attribute(
        "oauth.Token",
        "#[derive(::serde::Deserialize)]",
    )
    .field_attribute(
        "oauth.User.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )

    // record
    .type_attribute(
        "record.Record",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::records)]",
    )
    .type_attribute(
        "record.NewRecord",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::records)]",
    )
    .field_attribute(
        "record.Record.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid, serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "record.NewRecord.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid, serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )

    // user
    .type_attribute(
        "user.User",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::users)]",
    )
    .type_attribute(
        "user.NewUser",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::users)]",
    )
    .field_attribute(
        "user.User.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )

    // wx
    .type_attribute(
        "wx.SnsJscode2sessionRequest",
        "#[derive(::serde::Serialize)]",
    )
    .type_attribute(
        "wx.SnsJscode2sessionResponse",
        "#[derive(::serde::Deserialize)]",
    )
    .type_attribute(
        "wx.Error",
        "#[derive(::serde::Deserialize)]",
    )

    // common
    .field_attribute(
        "created_at",
        "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
    )
    .field_attribute(
        "updated_at",
        "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
    );
    build_config.compile(
        &[
            "./proto/lottery.proto",
            "./proto/oauth.proto",
            "./proto/record.proto",
            "./proto/user.proto",
            "./proto/wx.proto",
        ],
        &["./proto", "../proto/third_party"],
    )?;
    Ok(())
}
