use tonic_build::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_lottery()?;
    build_oauth()?;
    build_record()?;
    build_user()?;
    build_wx()?;
    Ok(())
}

fn build_lottery() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
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
            "type",
            "#[diesel(column_name = \"type_\")]",
        )
        .field_attribute(
            "created_at",
            "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
        )
        .field_attribute(
            "updated_at",
            "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
        );
    build_config.compile(
        &["./proto/lottery.proto"],
        &["./proto", "../proto/third_party"],
    )?;
    Ok(())
}
fn build_oauth() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure();
    build_config.compile(
        &["./proto/oauth.proto"],
        &["./proto", "../proto/third_party"],
    )?;
    Ok(())
}

fn build_record() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
        .type_attribute(
            "record.Record",
            "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
            #[diesel(table_name=crate::schema::records)]",
        )
        .type_attribute(
            "user.NewRecord",
            "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
            #[diesel(table_name=crate::schema::records)]",
        )
        .field_attribute(
            "created_at",
            "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
        )
        .field_attribute(
            "updated_at",
            "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
        );
    build_config.compile(
        &["./proto/record.proto"],
        &["./proto", "../proto/third_party"],
    )?;
    Ok(())
}

fn build_user() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
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
        .field_attribute(
            "created_at",
            "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
        )
        .field_attribute(
            "updated_at",
            "#[diesel(deserialize_as = ::empty_utils::diesel::timestamp::Timestamp)]",
        );
    build_config.compile(
        &["./proto/user.proto"],
        &["./proto", "../proto/third_party"],
    )?;
    Ok(())
}
fn build_wx() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
        .type_attribute(
            "wx.SnsJscode2sessionRequest",
            "#[derive(::serde::Serialize)]",
        )
        .type_attribute(
            "wx.SnsJscode2sessionResponse",
            "#[derive(::serde::Deserialize)]",
        );
    build_config.compile(&["./proto/wx.proto"], &["./proto", "../proto/third_party"])?;
    Ok(())
}
