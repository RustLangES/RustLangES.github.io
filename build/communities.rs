use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path};

#[derive(Serialize, Deserialize)]
struct CommunityItem {
    pub name: Vec<String>,
    pub description: String,
    pub link: String,
    pub icon: String,
    pub brand_src: String,
    pub brand_alt: String,
}

pub fn generate(path: &Path) {
    let folders = fs::read_dir(path).unwrap();
    let mut communities = Vec::new();

    for file in folders {
        let file = file.unwrap();
        let meta = file.metadata().unwrap();
        if meta.is_dir() {
            continue;
        }
        let file_path = file.path();
        let toml_str = fs::read_to_string(&file_path).unwrap();
        let toml_str = toml::from_str::<CommunityItem>(&toml_str).unwrap();
        communities.push((file_path, toml_str));
    }
    let mut other_file = fs::File::create("src/extras/other_communities.rs").unwrap();
    write!(
        other_file,
        "use crate::models::CommunityItem;\npub const OTHER_COMMUNITIES: &[CommunityItem] = &[\n"
    )
    .expect("No se pudo crear el archivo src/extras/other_communities.rs");

    let mut rust_file = fs::File::create("src/extras/rust_communities.rs").unwrap();
    write!(
        rust_file,
        "use crate::models::CommunityItem;\npub const RUST_COMMUNITIES: &[CommunityItem] = &[\n"
    )
    .expect("No se pudo crear el archivo src/extras/rust_communities.rs");

    for (_p, community) in communities {
        let mut output_file = if community.name.join("").to_lowercase().contains("rust") {
            &rust_file
        } else {
            &other_file
        };

        let CommunityItem {
            name,
            description,
            link,
            icon,
            brand_src,
            brand_alt,
        } = community;
        let brand_src = brand_src.replace("./", "/gen_assets/");

        write!(
            output_file,
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

    write!(other_file, "\n];").unwrap();
    write!(rust_file, "\n];").unwrap();
}
