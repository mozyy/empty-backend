fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure();
    // .out_dir("../protos")
    // .extern_path(".uuid", "::uuid")
    // .type_attribute(
    //     "lottery.v1.Lottery",
    //     "#[derive(::diesel::prelude::Queryable)]",
    // )
    // .type_attribute(
    //     "lottery.v1.NewLottery",
    //     "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]",
    // )
    // .type_attribute(
    //     "lottery.v1.NewLottery",
    //     "#[diesel(table_name = crate::schema::lotterys)]",
    // )
    // .field_attribute(
    //     "lottery.v1.NewLottery.type",
    //     "#[diesel(column_name = \"type_\")]",
    // )
    // .extern_path(
    //     ".google.protobuf.Timestamp",
    //     "::empty_utils::tonic::timestamp::Timestamp",
    // );
    build_config.compile(
        &[
            "proto/blog/blog.proto",
            "proto/file/file.proto",
            "proto/lottery/lottery.proto",
        ],
        &["proto", "third_party"],
    )?;
    Ok(())
}
