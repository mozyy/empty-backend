fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
    // lottery
    // .type_attribute(
    //     "lottery.Item",
    //     "#[derive(::diesel::FromSqlRow, ::diesel::AsExpression)]
    //     #[diesel(sql_type = ::lottery::schema::sql_types::Item)]",
    // )
    .type_attribute(
        "lottery.lottery.Item",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::items, belongs_to(crate::pb::lottery::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.lottery.NewItem",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::items, belongs_to(crate::pb::lottery::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.lottery.Remark",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::remarks, belongs_to(crate::pb::lottery::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.lottery.NewRemark",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::remarks, belongs_to(crate::pb::lottery::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.lottery.LotteryInfo",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::lotterys, belongs_to(crate::pb::oauth::oauth::User))]",
    )
    .field_attribute(
        "lottery.lottery.LotteryInfo.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "lottery.lottery.NewLotteryInfo",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::lotterys, belongs_to(crate::pb::oauth::oauth::User))]",
    )
    .field_attribute(
        "lottery.lottery.NewLotteryInfo.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "lottery.lottery.LotteryInfo.type",
        "#[diesel(column_name = \"type_\")]",
    )
    .field_attribute(
        "lottery.lottery.NewLotteryInfo.type",
        "#[diesel(column_name = \"type_\")]",
    )
    // record
    .type_attribute(
        "lottery.record.RecordInfo",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::records, belongs_to(crate::pb::oauth::oauth::User), belongs_to(crate::pb::lottery::lottery::Item), belongs_to(crate::pb::lottery::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.record.NewRecordInfo",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::records, belongs_to(crate::pb::oauth::oauth::User))]",
    )
    .type_attribute(
        "lottery.record.RecordRemark",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::record_remarks, belongs_to(crate::pb::lottery::record::RecordInfo, foreign_key = record_id))]",
    )
    .type_attribute(
        "lottery.record.NewRecordRemark",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::record_remarks, belongs_to(crate::pb::lottery::record::RecordInfo, foreign_key = record_id))]",
    )
    .field_attribute(
        "lottery.record.RecordInfo.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "lottery.record.NewRecordInfo.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )

    // oauth
    .type_attribute(
        "oauth.oauth.User",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable)]
    #[diesel(table_name=crate::schema::oauth::users)]",
    )
    .field_attribute(
        "oauth.oauth.User.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "oauth.oauth.Token",
        "#[derive(::serde::Deserialize)]",
    )
    .type_attribute(
        "oauth.oauth.Client",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::oauth::oauth_clients)]",
    )
    .field_attribute(
        "oauth.oauth.Client.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "oauth.oauth.NewClient",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::oauth::oauth_clients)]",
    )
    .type_attribute(
        "oauth.oauth.Config",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::oauth::oauth_configs)]",
    )
    .type_attribute(
        "oauth.oauth.NewConfig",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::oauth::oauth_configs)]",
    )
    .type_attribute(
        "oauth.oauth.Pattern",
        "#[derive(::diesel::FromSqlRow, ::diesel::AsExpression)]
        #[diesel(sql_type=crate::schema::oauth::sql_types::OauthPattern)]",
    )

    // wx
    .type_attribute(
        "wx.wx.SnsJscode2sessionRequest",
        "#[derive(::serde::Serialize)]",
    )
    .type_attribute(
        "wx.wx.SnsJscode2sessionResponse",
        "#[derive(::serde::Deserialize)]",
    )
    .type_attribute(
        "wx.wx.Error",
        "#[derive(::serde::Deserialize)]",
    )
    .type_attribute(
        "wx.user.User",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::wx::users, belongs_to(crate::pb::oauth::oauth::User))]",
    )
    .type_attribute(
        "wx.user.NewUser",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::wx::users, belongs_to(crate::pb::oauth::oauth::User))]",
    )
    .field_attribute(
        "wx.user.User.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "wx.user.User.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "wx.user.NewUser.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )

    // blog
    .type_attribute(
        "blog.blog.Blog",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::blog::blogs)]",
    )
    .type_attribute(
        "blog.blog.NewBlog",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::blog::blogs)]",
    )

    // oss
    .type_attribute(
        "oss.oss.Oss",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::oss::oss)]",
    )
    .type_attribute(
        "oss.oss.NewOss",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::oss::oss)]",
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
                "./proto/lottery/lottery.proto",
                "./proto/lottery/record.proto",
                "./proto/oauth/oauth.proto",
                "./proto/wx/wx.proto",
                "./proto/wx/user.proto",
                "./proto/blog/blog.proto",
                "./proto/oss/oss.proto",
            ],
            &["./proto", "../proto/third_party"],
        )?;
    Ok(())
}
