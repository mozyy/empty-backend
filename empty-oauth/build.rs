fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        // .out_dir("../protos")
        .compile_with_config(config, &["proto/oauth.proto"], &["./"])?;
    // tonic_build::compile_protos("proto/oauth.proto")?;
    Ok(())
}
