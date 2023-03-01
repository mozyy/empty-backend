fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos_dir = &std::env::current_dir()?.join("../protos");
    // Command::new("diesel")
    //     .args(["migration", "run"])
    //     .current_dir(current_dir)
    //     .spawn()
    //     .expect("msg1");
    // let file = File::create("./protos/user.proto").expect("msg22");
    // let stdio = Stdio::from(file);
    // Command::new("diesel_ext")
    //     .args(["-p"])
    //     .stdout(stdio)
    //     .spawn()?;
    tonic_build::configure()
        .out_dir(protos_dir)
        .compile(&[protos_dir.join("user.proto")], &[protos_dir])?;
    // tonic_build::compile_protos("../protos/user.proto")?;
    Ok(())
}
