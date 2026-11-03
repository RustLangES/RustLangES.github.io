mod communities;
mod projects;
mod top_projects;

use std::{fs, io::Write, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=extras");
    println!("cargo:rerun-if-changed=src/extras/top_projects.rs");

    let folders = fs::read_dir("extras").unwrap();
    if let Err(e) = fs::create_dir("src/extras")
        && e.kind() != std::io::ErrorKind::AlreadyExists
    {
        println!("{e:?}");
    }

    copy_dir_all("extras/proyectos/assets", "assets/gen_assets").unwrap();
    copy_dir_all("extras/comunidades/assets", "assets/gen_assets").unwrap();

    // Generate src/extras/mod.rs
    let mut out = fs::File::create("src/extras/mod.rs").unwrap();
    write!(out, "#[rustfmt::skip]\nmod other_communities;\nmod rust_communities;\n#[rustfmt::skip]\nmod projects;\n#[rustfmt::skip]\nmod top_projects;\npub use other_communities::*;\npub use rust_communities::*;\npub use projects::*;\npub use top_projects::*;\n").unwrap();

    let mut extra_projects = Vec::new();

    for folder in folders {
        let folder = folder.unwrap();
        let meta = folder.metadata().unwrap();
        if !meta.is_dir() {
            continue;
        }

        let mut path = std::env::current_dir().unwrap();
        path.push(folder.path());

        match folder.file_name().to_str().unwrap() {
            "comunidades" => communities::generate(&path),
            "proyectos" => extra_projects = projects::generate(&path),
            _ => {}
        }
    }

    top_projects::generate(&extra_projects);
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
