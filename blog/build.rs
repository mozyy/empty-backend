fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
        // .out_dir("../protos")
        .type_attribute(
            "blog.Blog",
            "#[derive(::diesel::prelude::Queryable, ::diesel::prelude::Identifiable, ::diesel::prelude::Selectable)]
            #[diesel(table_name=crate::schema::blogs)]",
        )
        .type_attribute(
            "blog.NewBlog",
            "#[derive(::diesel::prelude::Insertable, ::diesel::prelude::AsChangeset)]
            #[diesel(table_name=crate::schema::blogs)]",
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
        &["../proto/proto/blog/blog.proto"],
        &["../proto/proto", "../proto/third_party"],
    )?;
    Ok(())
}
