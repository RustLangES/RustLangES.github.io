use std::io::Write;
use std::{
    fs,
    fs::{DirEntry, Metadata},
    path::PathBuf,
};

fn main() {
    println!("cargo:rerun-if-changed=extras");

    let folders = fs::read_dir("extras").unwrap();
    if let Err(e) = fs::create_dir("src/extras") {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            println!("{e:?}");
        }
    }

    // Generate src/extras/mod.rs
    let mut out = fs::File::create("src/extras/mod.rs").unwrap();
    write!(out, "#[rustfmt::skip]\nmod comunities;\n#[rustfmt::skip]\nmod projects;\npub use comunities::*;\npub use projects::*;").unwrap();

    for folder in folders {
        let folder = folder.unwrap();
        let meta = folder.metadata().unwrap();
        if !meta.is_dir() {
            continue;
        }

        let mut path = std::env::current_dir().unwrap();
        path.push(folder.path());

        match folder.file_name().to_str().unwrap() {
            "comunidades" => generate_comunity(path),
            "proyectos" => generate_projects(path),
            _ => {}
        }
    }
}

fn generate_comunity(path: PathBuf) {
    let folders = fs::read_dir(path.as_path()).unwrap();
    let mut comunities = Vec::new();

    for file in folders {
        let file = file.unwrap();
        let meta = file.metadata().unwrap();
        if meta.is_dir() {
            continue;
        }
        let file_path = file.path();
        let toml_str = fs::read_to_string(&file_path).unwrap();
        comunities.push((file_path, toml_str));
    }
    let mut out = fs::File::create("src/extras/comunities.rs").unwrap();
    write!(out, "use crate::models::CommunityItem;\npub const OTHER_COMUNITIES: &[CommunityItem] = &[\n").unwrap();
    for (_p, t) in comunities {
        write!(out, r#"
    CommunityItem {{
        {t}
    }},"#).unwrap();
    }
    write!(out, "\n];").unwrap();
}

fn iter_dir(path: PathBuf, mut callback: impl FnMut(DirEntry, Metadata)) {
    println!("Read Dir: {path:?}");
    let folders = fs::read_dir(path.as_path()).unwrap();
    for folder in folders {
        let folder = folder.unwrap();
        let meta = folder.metadata().unwrap();
        callback(folder, meta);
    }
}

fn generate_projects(path: PathBuf) {
    let mut projects = Vec::new();
    iter_dir(path, |folder, meta| {
        if meta.is_file() {
            return;
        }
        let category = folder.file_name();
        let category = category.to_str().unwrap();
        println!("Category: {category}");

        let category = category.to_string();
        iter_dir(folder.path(), |file, meta| {
            if meta.is_dir() {
                return;
            }
            let file_path = file.path();
            let toml_str = fs::read_to_string(&file_path).unwrap();
            projects.push((category.clone(), file_path, toml_str));
        });
    });

    let mut out = fs::File::create("src/extras/projects.rs").unwrap();
    write!(out, "use crate::models::ProjectItem;\npub const COMUNITY_PROJECTS: &[ProjectItem] = &[\n").unwrap();
    for (_c, _p, t) in projects {
    // name: ,
    // description: ,
    // link: ,
    // brand_src: ,
    // button_link: ,
    // button_text: ,
    // brand_as_letter: 
    // button_bg_color: ,
        write!(out, r#"
    ProjectItem {{
        {t}
    }},"#).unwrap();
    }
    write!(out, "\n];").unwrap();
}
