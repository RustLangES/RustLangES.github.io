use serde::{Deserialize, Serialize};
use std::{
    fs,
    fs::{DirEntry, Metadata},
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=extras");

    let folders = fs::read_dir("extras").unwrap();
    if let Err(e) = fs::create_dir("src/extras") {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            println!("{e:?}");
        }
    }

    copy_dir_all("extras/proyectos/assets", "assets/gen_assets").unwrap();
    copy_dir_all("extras/comunidades/assets", "assets/gen_assets").unwrap();

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
        let toml_str = toml::from_str::<CommunityItem>(&toml_str).unwrap();
        comunities.push((file_path, toml_str));
    }
    let mut out = fs::File::create("src/extras/comunities.rs").unwrap();
    write!(
        out,
        "use crate::models::CommunityItem;\npub const OTHER_COMUNITIES: &[CommunityItem] = &[\n"
    )
    .unwrap();
    for (_p, t) in comunities {
        let CommunityItem {
            name,
            description,
            link,
            icon,
            brand_src,
            brand_alt,
        } = t;
        let brand_src = brand_src.replace("./", "/gen_assets/");
        write!(
            out,
            r#"
    CommunityItem {{
        name: &{name:?},
        description: "{description}",
        link: "{link}",
        icon: "{icon}",
        brand_src: "{brand_src}",
        brand_alt: "{brand_alt}",
    }},"#
        )
        .unwrap();
    }
    write!(out, "\n];").unwrap();
}

fn iter_dir(path: PathBuf, mut callback: impl FnMut(DirEntry, Metadata)) {
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

        let category = category.to_string();
        iter_dir(folder.path(), |file, meta| {
            if meta.is_dir() {
                return;
            }
            let file_path = file.path();

            if !file_path.extension().is_some_and(|e| e == "toml") {
                let file_name = file.file_name();
                let file_name = file_name.to_str().unwrap();
                // Copy images or other files
                fs::copy(&file_path, format!("assets/gen_assets/{file_name}")).unwrap();
                return;
            }

            let toml_str = fs::read_to_string(&file_path).unwrap();
            let toml_str = toml::from_str::<ProjectItem>(&toml_str).unwrap();
            projects.push((category.clone(), file_path, toml_str));
        });
    });

    let mut out = fs::File::create("src/extras/projects.rs").unwrap();
    write!(
        out,
        "use crate::models::ProjectItem;\npub const COMUNITY_PROJECTS: &[ProjectItem] = &[\n"
    )
    .unwrap();
    for (c, _p, t) in projects {
        let ProjectItem {
            name,
            description,
            link,
            brand_src,
            button_link,
            button_text,
            brand_as_letter,
            button_bg_color,
        } = t;
        let brand_src = brand_src.replace("./", "/gen_assets/");
        write!(
            out,
            r#"
    ProjectItem {{
        name: &{name:?},
        category: "{c}",
        description: "{description}",
        link: "{link}",
        brand_src: "{brand_src}",
        button_link: "{button_link}",
        button_text: "{button_text}",
        brand_as_letter: {brand_as_letter},
        button_bg_color: "{button_bg_color}",
    }},"#
        )
        .unwrap();
    }
    write!(out, "\n];").unwrap();
}

#[derive(Serialize, Deserialize)]
struct CommunityItem {
    pub name: Vec<String>,
    pub description: String,
    pub link: String,
    pub icon: String,
    pub brand_src: String,
    pub brand_alt: String,
}

#[derive(Serialize, Deserialize)]
struct ProjectItem {
    pub name: Vec<String>,
    pub description: String,
    pub link: String,
    pub brand_src: String,
    pub button_link: String,
    pub button_text: String,
    pub brand_as_letter: bool,
    pub button_bg_color: String,
}
