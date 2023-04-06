use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let proto = Path::new("../proto");
    // let build_config = tonic_build::configure()
    //     // .out_dir("../protos")
    //     ;
    // build_config.compile(
    //     &[proto.join("proto/blog/blog.proto")],
    //     &[
    //         proto.join("proto"),
    //         proto.join("third_party"),
    //         "~/.local/include".into(),
    //     ],
    // )?;
    Ok(())
}
