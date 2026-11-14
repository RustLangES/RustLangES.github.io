use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::Path};

#[derive(Serialize, Deserialize)]
struct ChannelItem {
    pub name: String,
    pub description: String,
    pub avatar: String,
    pub youtube: Option<String>,
    pub twitch: Option<String>,
}

pub fn generate(path: &Path) {
    let mut files = fs::read_dir(path)
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "toml"))
        .collect::<Vec<_>>();
    files.sort();

    let mut out = fs::File::create("src/extras/channels.rs").unwrap();
    write!(
        out,
        "use crate::models::ChannelItem;\npub const CHANNELS: &[ChannelItem] = &["
    )
    .expect("No se pudo crear el archivo src/extras/channels.rs");

    for file_path in files {
        let toml_str = fs::read_to_string(&file_path).unwrap();
        let ChannelItem {
            name,
            description,
            avatar,
            youtube,
            twitch,
        } = toml::from_str::<ChannelItem>(&toml_str).unwrap();
        let avatar = avatar.replace("./", "/gen_assets/");

        write!(
            out,
            r#"
    ChannelItem {{
        name: {name:?},
        description: {description:?},
        avatar: {avatar:?},
        youtube: {youtube:?},
        twitch: {twitch:?},
    }},"#
        )
        .unwrap();
    }

    write!(out, "\n];\n").unwrap();
}
