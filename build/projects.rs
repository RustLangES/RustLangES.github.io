use serde::{Deserialize, Serialize};
use std::{
    fs,
    fs::{DirEntry, Metadata},
    io::Write,
    path::Path,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectItem {
    pub name: Vec<String>,
    pub description: String,
    pub link: String,
    pub brand_src: String,
    pub button_link: String,
    pub button_text: String,
    pub brand_as_letter: bool,
    pub button_bg_color: String,
}

fn iter_dir(path: &Path, mut callback: impl FnMut(DirEntry, Metadata)) {
    let folders = fs::read_dir(path).unwrap();
    for folder in folders {
        let folder = folder.unwrap();
        let meta = folder.metadata().unwrap();
        callback(folder, meta);
    }
}

pub fn generate(path: &Path) -> Vec<ProjectItem> {
    let mut projects = Vec::new();
    iter_dir(path, |folder, meta| {
        if meta.is_file() {
            return;
        }
        let category = folder.file_name();
        let category = category.to_str().unwrap();

        let category = category.to_string();
        iter_dir(&folder.path(), |file, meta| {
            if meta.is_dir() {
                return;
            }
            let file_path = file.path();

            if file_path.extension().is_none_or(|e| e != "toml") {
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
    let mut items = Vec::with_capacity(projects.len());
    for (c, _p, t) in projects {
        items.push(t.clone());
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
    items
}
