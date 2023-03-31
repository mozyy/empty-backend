use cargo_toml::Manifest;
use clap::Parser;
use std::{
    env::current_dir,
    error::Error,
    ffi::OsStr,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Read, Write},
};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "empty")]
enum EmptyCli {
    New {
        #[arg(long)]
        camel_case: Option<String>,
        #[arg(required = true)]
        name: String,
    },
}

pub fn cli() -> Result<(), Box<dyn Error>> {
    let EmptyCli::New { name, camel_case } = EmptyCli::parse();
    // default camel_case is UpperCamelCase
    let camel_case = camel_case.unwrap_or_else(|| {
        let name = &name.to_owned();
        let mut chars = name.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    });
    let mut current = current_dir()?;
    let parent = loop {
        if Some(OsStr::new("empty-backend")) == current.file_name() {
            break current;
        }
        if let Some(path) = current.parent() {
            current = path.to_path_buf();
        } else {
            panic!("should inner empty-backend dir")
        }
    };
    let source_dir = parent.join("empty-template");
    let target_file = format!("empty-{name}");

    WalkDir::new(source_dir)
        .into_iter()
        .try_for_each(|entry| -> Result<(), Box<dyn Error>> {
            let entry = entry?;
            let file_name = &entry.path();
            let target_name = file_name
                .to_str()
                .ok_or_else(|| "file name to str error!")?
                .replace("template", &name);
            if entry.path().is_file() {
                let source_file = File::open(file_name)?;
                let target_file = File::create(target_name)?;
                let reader = BufReader::new(source_file);
                let mut writer = BufWriter::new(target_file);
                for line in reader.lines() {
                    let line = line?;
                    let line = line.replace("template", &name);
                    let line = line.replace("Template", &camel_case);
                    writeln!(writer, "{line}")?;
                }
            } else {
                fs::create_dir(target_name)?;
            }
            Ok(())
        })?;
    let cargo_file_name = parent.join("Cargo.toml");
    let cargo_file = File::open(&cargo_file_name)?;
    let mut reader = BufReader::new(cargo_file);
    let mut cargo_string = String::new();
    reader.read_to_string(&mut cargo_string)?;
    let mut manifest = Manifest::from_str(&cargo_string)?;
    if let Some(workspace) = manifest.workspace.as_mut() {
        workspace.members.push(target_file.clone());
    }
    let s = toml::to_string(&manifest)?;
    let mut cargo_file = File::create(&cargo_file_name)?;
    cargo_file.write_all(s.as_bytes())?;
    println!("generate new:{target_file} from  {name}, {camel_case}");

    Ok(())
}
