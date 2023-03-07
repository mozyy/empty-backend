fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::configure()
    // .out_dir("../protos")
    // .compile(&["proto/registry.proto"], &["./"])?;
    tonic_build::compile_protos("proto/registry.proto")?;
    Ok(())
}
