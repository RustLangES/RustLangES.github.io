use rand::seq::SliceRandom;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, io::Write, path::Path, time::Duration};

use crate::projects::ProjectItem;

#[derive(Serialize, Deserialize)]
struct TopProject {
    name: String,
    description: String,
    link: String,
    stars: Option<u32>,
    contributors: Vec<GithubContributor>,
}

#[derive(Serialize, Deserialize)]
struct GithubSearch {
    items: Vec<GithubRepo>,
}

#[derive(Serialize, Deserialize)]
struct GithubRepo {
    name: String,
    full_name: String,
    html_url: String,
    description: Option<String>,
    stargazers_count: u32,
}

#[derive(Serialize, Deserialize)]
struct GithubContributor {
    login: String,

    avatar_url: String,

    #[serde(rename = "type")]
    kind: String,
}

pub fn generate(extra_projects: &[ProjectItem]) {
    if Path::new("src/extras/top_projects.rs").exists() {
        return;
    }

    let projects = fetch().unwrap_or_else(|e| random_extra_projects(extra_projects));

    write(&projects);
}

fn fetch() -> Result<Vec<TopProject>, Box<dyn Error>> {
    let mut headers = HeaderMap::new();

    if let Ok(token) = env::var("GITHUB_TOKEN") {
        headers.insert(AUTHORIZATION, format!("Bearer {token}").parse()?);
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent("RustLangES")
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()?;

    let search: GithubSearch = client
        .get("https://api.github.com/search/repositories?q=org:RustLangES&sort=stars&order=desc&per_page=3")
        .send()?
        .error_for_status()?
        .json()?;

    let mut projects = Vec::new();
    for repo in search.items.into_iter().take(3) {
        let contributors = fetch_contributors(&client, &repo.full_name, 10)?;

        let excluded: Vec<String> = if repo.full_name == "RustLangES/rust-book-es" {
            fetch_contributors(&client, "rust-lang/book", 10)?
                .into_iter()
                .map(|c| c.login)
                .collect()
        } else {
            Vec::new()
        };

        projects.push(TopProject {
            name: repo.name,
            description: repo.description.unwrap_or_default(),
            link: repo.html_url,
            stars: Some(repo.stargazers_count),
            contributors: contributors
                .into_iter()
                .filter(|c| c.kind == "User" && !excluded.contains(&c.login))
                .take(4)
                .collect(),
        });
    }

    Ok(projects)
}

fn fetch_contributors(
    client: &reqwest::blocking::Client,
    full_name: &str,
    per_page: u8,
) -> Result<Vec<GithubContributor>, Box<dyn Error>> {
    Ok(client
        .get(format!(
            "https://api.github.com/repos/{full_name}/contributors?per_page={per_page}"
        ))
        .send()?
        .error_for_status()?
        .json()?)
}

fn random_extra_projects(extra_projects: &[ProjectItem]) -> Vec<TopProject> {
    let mut projects = extra_projects
        .iter()
        .map(|p| TopProject {
            name: p
                .name
                .join(" ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
            description: p.description.clone(),
            link: p.link.clone(),
            stars: None,
            contributors: github_user_from_link(&p.button_link).into_iter().collect(),
        })
        .collect::<Vec<_>>();

    projects.shuffle(&mut rand::rng());
    projects.truncate(3);
    projects
}

fn github_user_from_link(link: &str) -> Option<GithubContributor> {
    let login = link
        .strip_prefix("https://github.com/")?
        .trim_end_matches('/');
    if login.is_empty() || login.contains('/') {
        return None;
    }
    Some(GithubContributor {
        login: login.to_string(),
        avatar_url: format!("https://github.com/{login}.png"),
        kind: "User".to_string(),
    })
}

fn write(projects: &[TopProject]) {
    let mut out = fs::File::create("src/extras/top_projects.rs")
        .expect("No se pudo crear el archivo top_projects.rs");
    write!(
        out,
        "use crate::models::{{GithubUser, TopProject}};\npub const TOP_PROJECTS: &[TopProject] = &["
    )
    .expect("No se pudo escribir top_projects.rs");

    for project in projects {
        let TopProject {
            name,
            description,
            link,
            stars,
            contributors,
        } = project;
        let contributors = contributors
            .iter()
            .map(|c| {
                format!(
                    "GithubUser {{ username: {:?}, avatar: {:?} }}",
                    c.login, c.avatar_url
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            out,
            r#"
    TopProject {{
        name: {name:?},
        description: {description:?},
        link: {link:?},
        stars: {stars:?},
        contributors: &[{contributors}],
    }},"#
        )
        .expect("No se pudo escribir un proyecto en top_projects.rs");
    }
    write!(out, "\n];").expect("No se pudo cerrar la lista en top_projects.rs");
}
