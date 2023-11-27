fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
    // lottery
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
        #[diesel(table_name=crate::schema::lottery::lotterys, belongs_to(crate::pb::auth::auth::User))]",
    )
    .field_attribute(
        "lottery.lottery.LotteryInfo.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "lottery.lottery.NewLotteryInfo",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::lotterys, belongs_to(crate::pb::auth::auth::User))]",
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
        #[diesel(table_name=crate::schema::lottery::records, belongs_to(crate::pb::auth::auth::User), belongs_to(crate::pb::lottery::lottery::Item), belongs_to(crate::pb::lottery::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.record.NewRecordInfo",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::records, belongs_to(crate::pb::auth::auth::User))]",
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
    // template
    .type_attribute(
        "lottery.template.Template",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Associations, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::lottery::templates, belongs_to(crate::pb::lottery::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.template.NewTemplate",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::lottery::templates)]",
    )
    // favorite
    .type_attribute(
        "lottery.favorite.Favorite",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Associations, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::lottery::favorites, belongs_to(crate::pb::auth::auth::User), belongs_to(crate::pb::lottery::lottery::LotteryInfo, foreign_key = lottery_id))]",
    )
    .type_attribute(
        "lottery.favorite.NewFavorite",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::lottery::favorites, belongs_to(crate::pb::auth::auth::User))]",
    )
    .field_attribute(
        "lottery.favorite.Favorite.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "lottery.favorite.NewFavorite.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )

    // auth
    .type_attribute(
        "auth.auth.Resource",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::auth::resources)]",
    )
    .field_attribute(
        "auth.auth.Resource.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "auth.auth.Resource.client_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "auth.auth.Resource.until",
        "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
    )
    .type_attribute(
        "auth.auth.NewResource",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::auth::resources)]",
    )
    .field_attribute(
        "auth.auth.NewResource.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "auth.auth.NewResource.client_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "auth.auth.NewResource.until",
        "#[diesel(serialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
    )
    .type_attribute(
        "auth.auth.User",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable)]
    #[diesel(table_name=crate::schema::auth::users)]",
    )
    .field_attribute(
        "auth.auth.User.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "auth.auth.Client",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::auth::clients)]",
    )
    .field_attribute(
        "auth.auth.Client.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "auth.auth.NewClient",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::auth::clients)]",
    )
    .type_attribute(
        "auth.auth.Config",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::auth::configs)]",
    )
    .type_attribute(
        "auth.auth.NewConfig",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::auth::configs)]",
    )
    .type_attribute(
        "auth.auth.Pattern",
        "#[derive(::diesel::FromSqlRow, ::diesel::AsExpression)]
        #[diesel(sql_type=crate::schema::auth::sql_types::Pattern)]",
    )

    // websocket
    .type_attribute(
        "websocket.client.User",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::websocket::users, belongs_to(crate::pb::auth::auth::User))]",
    )
    .type_attribute(
        "websocket.client.NewUser",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::websocket::users, belongs_to(crate::pb::auth::auth::User))]",
    )
    .field_attribute(
        "websocket.client.User.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "websocket.client.NewUser.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
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
        #[diesel(table_name=crate::schema::wx::users, belongs_to(crate::pb::auth::auth::User))]",
    )
    .type_attribute(
        "wx.user.NewUser",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::wx::users, belongs_to(crate::pb::auth::auth::User))]",
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

    // user
    .type_attribute(
        "user.auth.AuthorizationCode",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::user::authorization_codes, belongs_to(crate::pb::user::user::User), belongs_to(crate::pb::user::auth::Client))]",
    )
    .field_attribute(
        "user.auth.AuthorizationCode.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.auth.AuthorizationCode.client_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.auth.AuthorizationCode.until",
        "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
    )
    .type_attribute(
        "user.auth.NewAuthorizationCode",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::user::authorization_codes, belongs_to(crate::pb::user::user::User), belongs_to(crate::pb::user::auth::Client))]",
    )
    .field_attribute(
        "user.auth.NewAuthorizationCode.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.auth.NewAuthorizationCode.client_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.auth.NewAuthorizationCode.until",
        "#[diesel(serialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
    )
    .type_attribute(
        "user.auth.RefreshResource",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::user::refresh_resources, belongs_to(crate::pb::user::user::User), belongs_to(crate::pb::user::auth::Client))]",
    )
    .field_attribute(
        "user.auth.RefreshResource.user_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.auth.RefreshResource.client_id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.auth.RefreshResource.until",
        "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
    )
    .type_attribute(
        "user.auth.NewRefreshResource",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
        #[diesel(table_name=crate::schema::user::refresh_resources, belongs_to(crate::pb::user::user::User), belongs_to(crate::pb::user::auth::Client))]",
    )
    .field_attribute(
        "user.auth.NewRefreshResource.user_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.auth.NewRefreshResource.client_id",
        "#[diesel(serialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .field_attribute(
        "user.auth.NewRefreshResource.until",
        "#[diesel(serialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
    )
    .type_attribute(
        "user.auth.Client",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::user::clients)]",
    )
    .field_attribute(
        "user.auth.Client.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "user.auth.NewClient",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::user::clients)]",
    )
    .type_attribute(
        "user.auth.Config",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
        #[diesel(table_name=crate::schema::user::configs)]",
    )
    .type_attribute(
        "user.auth.NewConfig",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
        #[diesel(table_name=crate::schema::user::configs)]",
    )
    .type_attribute(
        "user.auth.Pattern",
        "#[derive(::diesel::FromSqlRow, ::diesel::AsExpression)]
        #[diesel(sql_type=crate::schema::user::sql_types::Pattern)]",
    )
    .type_attribute(
        "user.user.Info",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable)]
    #[diesel(table_name=crate::schema::user::infos)]",
    )
    .type_attribute(
        "user.user.NewInfo",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
    #[diesel(table_name=crate::schema::user::infos)]",
    )
    .type_attribute(
        "user.user.Mobile",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable)]
    #[diesel(table_name=crate::schema::user::mobiles)]",
    )
    .type_attribute(
        "user.user.NewMobile",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
    #[diesel(table_name=crate::schema::user::mobiles)]",
    )
    .type_attribute(
        "user.user.Weixin",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable)]
    #[diesel(table_name=crate::schema::user::weixins)]",
    )
    .type_attribute(
        "user.user.NewWeixin",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
    #[diesel(table_name=crate::schema::user::weixins)]",
    )
    .type_attribute(
        "user.user.User",
        "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Associations)]
    #[diesel(table_name=crate::schema::user::users, belongs_to(crate::pb::user::user::Info), belongs_to(crate::pb::user::user::Mobile), belongs_to(crate::pb::user::user::Weixin))]",
    )
    .field_attribute(
        "user.user.User.id",
        "#[diesel(deserialize_as = ::empty_utils::tonic::uuid::Uuid)]",
    )
    .type_attribute(
        "user.user.NewUser",
        "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset, ::diesel::prelude::Associations)]
    #[diesel(table_name=crate::schema::user::users, belongs_to(crate::pb::user::user::Info), belongs_to(crate::pb::user::user::Mobile), belongs_to(crate::pb::user::user::Weixin))]",
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
                "./proto/lottery/template.proto",
                "./proto/lottery/favorite.proto",
                "./proto/auth/auth.proto",
                "./proto/auth/auth.proto",
                "./proto/user/user.proto",
                "./proto/user/auth.proto",
                "./proto/websocket/client.proto",
                "./proto/wx/wx.proto",
                "./proto/wx/user.proto",
                "./proto/blog/blog.proto",
                "./proto/oss/oss.proto",
            ],
            &["./proto", "../proto/third_party"],
        )?;
    Ok(())
}
