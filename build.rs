use std::{fs::read_dir, path::PathBuf};

fn get_c_sources(file: PathBuf, vec: &mut Vec<PathBuf>) {
    if file.is_dir() {
        read_dir(file).unwrap().for_each(|file| get_c_sources(file.unwrap().path(), vec));
    } else {
        file.extension().map(|ext| {
            if ext == "c" {
                vec.push(file.clone());
            }
        });
    }
}

fn main() {
    let mut cfiles = Vec::new();
    get_c_sources("src/cproj/src".into(), &mut cfiles);

    assert!(cfiles.len() > 0);

    for file in &cfiles {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    cc::Build::new()
        .include("src/cproj/include")
        .files(cfiles)
        .compile("cproj");
}