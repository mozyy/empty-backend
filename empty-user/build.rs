use std::{
    fs::{self, File},
    process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("diesel").args(["migration", "run"]).spawn()?;
    let file = File::create("../protos/user.proto").unwrap();
    let stdio = Stdio::from(file);
    Command::new("diesel_ext")
        .args(["-p"])
        .stdout(stdio)
        .spawn()?;
    tonic_build::configure()
        .out_dir("../protos")
        .compile(&["../protos/user.proto"], &["../protos"])?;
    // tonic_build::compile_protos("../protos/user.proto")?;
    Ok(())
}
