fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
        // .out_dir("../protos")
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
            "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
            #[diesel(table_name=crate::schema::lotterys)]",
        )
        .type_attribute(
            "lottery.NewLottery",
            "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
            #[diesel(table_name=crate::schema::lotterys)]",
        )
        .field_attribute(
            "type",
            "#[diesel(column_name = \"type_\")]",
        )
        .field_attribute(
            "created_at",
            "#[diesel(deserialize_as = ::empty_utils::tonic::timestamp::Timestamp)]",
        )
        .field_attribute(
            "updated_at",
            "#[diesel(deserialize_as = ::empty_utils::tonic::timestamp::Timestamp)]",
        )
        .extern_path(
            ".google.protobuf.Timestamp",
            "::empty_utils::tonic::timestamp::Timestamp",
        );
    build_config.compile(
        &["../proto/proto/lottery/lottery.proto"],
        &["../proto/proto", "../proto/third_party"],
    )?;
    Ok(())
}
