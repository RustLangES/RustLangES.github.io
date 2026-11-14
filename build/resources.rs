use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path};

#[derive(Serialize, Deserialize)]
struct ResourceItem {
    pub name: String,
    pub description: String,
    pub link: String,
    pub language: String,
    pub kinds: Vec<String>,
    #[serde(default)]
    pub featured: bool,
}

pub fn generate(path: &Path) {
    let mut files = fs::read_dir(path)
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .collect::<Vec<_>>();
    files.sort();

    let mut out = fs::File::create("src/extras/resources.rs").unwrap();
    write!(
        out,
        "use crate::models::ResourceItem;\npub const RESOURCES: &[ResourceItem] = &["
    )
    .expect("No se pudo crear el archivo src/extras/resources.rs");

    for file_path in files {
        let toml_str = fs::read_to_string(&file_path).unwrap();
        let ResourceItem {
            name,
            description,
            link,
            language,
            kinds,
            featured,
        } = toml::from_str::<ResourceItem>(&toml_str).unwrap();

        write!(
            out,
            r#"
    ResourceItem {{
        name: {name:?},
        description: {description:?},
        link: {link:?},
        language: {language:?},
        kinds: &{kinds:?},
        featured: {featured},
    }},"#
        )
        .unwrap();
    }

    write!(out, "\n];\n").unwrap();
}
