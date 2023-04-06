fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_config = tonic_build::configure()
        // .out_dir("../protos")
        ;
    build_config.compile(
        &["proto/blog/blog.proto"],
        &["proto", "third_party", "~/.local/include"],
    )?;
    Ok(())
}
