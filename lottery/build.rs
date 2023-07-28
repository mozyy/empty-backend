fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
    // lottery
    // .type_attribute(
    //     "lottery.Item",
    //     "#[derive(::diesel::FromSqlRow, ::diesel::AsExpression)]
    //     #[diesel(sql_type = crate::schema::sql_types::Item)]",
    // )
    .type_attribute(
        "lottery.Item",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::items, belongs_to(crate::pb::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.NewItem",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::items, belongs_to(crate::pb::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.Remark",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::remarks, belongs_to(crate::pb::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.NewRemark",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::remarks, belongs_to(crate::pb::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.LotteryInfo",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lotterys, belongs_to(crate::pb::oauth::User))]",
    )
    .field_attribute(
        "lottery.LotteryInfo.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "lottery.NewLotteryInfo",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lotterys, belongs_to(crate::pb::oauth::User))]",
    )
    .field_attribute(
        "lottery.NewLotteryInfo.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "lottery.LotteryInfo.type",
        "#[diesel(column_name = \"type_\")]",
    )
    .field_attribute(
        "lottery.NewLotteryInfo.type",
        "#[diesel(column_name = \"type_\")]",
    )

    // oauth
    .type_attribute(
        "oauth.User",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable)]
    #[diesel(table_name=crate::schema::users)]",
    )
    .field_attribute(
        "oauth.User.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "oauth.Token",
        "#[derive(::serde::Deserialize)]",
    )
    .type_attribute(
        "oauth.Client",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::oauth_clients)]",
    )
    .field_attribute(
        "oauth.Client.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "oauth.NewClient",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::oauth_clients)]",
    )
    .type_attribute(
        "oauth.Config",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::oauth_configs)]",
    )
    .type_attribute(
        "oauth.NewConfig",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::oauth_configs)]",
    )
    .type_attribute(
        "oauth.Pattern",
        "#[derive(::diesel::FromSqlRow, ::diesel::AsExpression)]
        #[diesel(sql_type = crate::schema::sql_types::OauthPattern)]",
    )


    // record
    .type_attribute(
        "record.RecordInfo",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::records, belongs_to(crate::pb::oauth::User))]",
    )
    .type_attribute(
        "record.NewRecordInfo",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::records, belongs_to(crate::pb::oauth::User))]",
    )
    .type_attribute(
        "record.RecordRemark",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::record_remarks, belongs_to(crate::pb::record::RecordInfo, foreign_key = record_id))]",
    )
    .type_attribute(
        "record.NewRecordRemark",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::record_remarks, belongs_to(crate::pb::record::RecordInfo, foreign_key = record_id))]",
    )
    .field_attribute(
        "record.RecordInfo.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "record.NewRecordInfo.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )

    // user
    .type_attribute(
        "user.WxUser",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::wx_users, belongs_to(crate::pb::oauth::User))]",
    )
    .type_attribute(
        "user.NewWxUser",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::wx_users, belongs_to(crate::pb::oauth::User))]",
    )
    .field_attribute(
        "user.WxUser.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.WxUser.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.NewWxUser.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
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
    build_config
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
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
