

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::configure()
    //     .out_dir("../protos")
    //     .compile(&["register.proto"], &["./"])?;
    tonic_build::compile_protos("register.proto")?;
    Ok(())
}
