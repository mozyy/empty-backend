fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
        // .out_dir("../protos")
        .type_attribute(
            "lottery.v1.Item",
            "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Associations, ::diesel::prelude::Selectable)]
            #[diesel(table_name=crate::schema::items)]
            #[diesel(belongs_to(LotteryInfo, foreign_key = lottery_id))]",
        )
        .type_attribute(
            "lottery.v1.NewItem",
            "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
            #[diesel(table_name=crate::schema::items)]",
        )
        .type_attribute(
            "lottery.v1.Remark",
            "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Associations, ::diesel::prelude::Selectable)]
            #[diesel(table_name=crate::schema::remarks,belongs_to(LotteryInfo, foreign_key = lottery_id))]",
        )
        .type_attribute(
            "lottery.v1.NewRemark",
            "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
            #[diesel(table_name=crate::schema::remarks)]",
        )
        .type_attribute(
            "lottery.v1.LotteryInfo",
            "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
            #[diesel(table_name=crate::schema::lotterys)]",
        )
        .type_attribute(
            "lottery.v1.NewLotteryInfo",
            "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
            #[diesel(table_name=crate::schema::lotterys)]",
        )
        .field_attribute(
            "lottery.v1.LotteryInfo.type",
            "#[diesel(column_name = \"type_\")]",
        )
        .field_attribute(
            "lottery.v1.NewLotteryInfo.type",
            "#[diesel(column_name = \"type_\")]",
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
