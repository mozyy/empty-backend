fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::configure()
    //     .out_dir("../protos")
    //     .compile(&["proto/register.proto"], &["./"])?;
    tonic_build::compile_protos("proto/template.proto")?;
    Ok(())
}
