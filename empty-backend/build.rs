use std::{
    error::Error,
    fs::{self, DirEntry, ReadDir},
    path::Path,
};

struct DirIter {
    entries: Vec<ReadDir>,
}
impl DirIter {
    fn new<P>(path: P) -> DirIter
    where
        P: AsRef<Path>,
    {
        let root = fs::read_dir(path).unwrap();
        DirIter {
            entries: vec![root],
        }
    }
}

impl Iterator for DirIter {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.entries.last_mut() {
            if let Some(Ok(next)) = entry.next() {
                if next.path().is_dir() {
                    if let Ok(e) = fs::read_dir(next.path()) {
                        self.entries.push(e);
                    }
                }
                return Some(next);
            } else {
                self.entries.pop();
            }
        }
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let protos: Vec<_> = DirIter::new("../protos/proto")
        .filter(|e| match e.path().extension() {
            Some(ext) => ext == "proto",
            None => false,
        })
        .map(|e| e.path())
        .collect();
    println!("protos:{:#?}", protos);
    tonic_build::configure()
        // .out_dir("src/protos")
        .type_attribute("*", "#[derive(Queryable)]")
        .compile(&protos, &["../protos/proto", "../protos/third_party"])?;
    Ok(())
}
