use reqwest::{
    StatusCode,
    blocking::Client,
    header::{AUTHORIZATION, HeaderMap},
};
use serde::{Deserialize, de::DeserializeOwned};
use std::{cmp::Reverse, collections::HashMap, env, error::Error, fs};

const UPSTREAMS: &[(&str, &str)] = &[
    ("rust-book-es", "rust-lang/book"),
    ("rust-embedded-book", "rust-embedded/book"),
    ("rust-para-dotnet-devs", "microsoft/rust-for-dotnet-devs"),
];

const NOT_COMMUNITY_MEMBERS: &[&str] = &["actions-user", "claude"];

#[derive(Deserialize)]
struct Repo {
    name: String,
    full_name: String,
    fork: bool,
}

#[derive(Deserialize)]
struct RepoContributor {
    login: String,
    contributions: u32,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Deserialize)]
struct Contributor {
    login: String,
    name: Option<String>,
    #[serde(rename = "avatar_url")]
    avatar: String,
    location: Option<String>,
    #[serde(rename = "twitter_username")]
    twitter: Option<String>,
    #[serde(default)]
    contributions: u32,
}

pub fn generate() {
    println!("cargo:rerun-if-changed=src/extras/contributors.rs");
    println!("cargo:rerun-if-env-changed=COLLABORATORS_API_TOKEN");
    println!("cargo:rerun-if-env-changed=GITHUB_TOKEN");

    if fs::read_to_string("src/extras/contributors.rs").is_ok_and(|code| !code.contains("&[]")) {
        return;
    }

    let list = match fetch() {
        Ok(contributors) => format!("{contributors:#?}"),
        Err(e) => {
            println!(
                "cargo:warning=No se pudo obtener la lista de contribuidores: {e}. Define COLLABORATORS_API_TOKEN o GITHUB_TOKEN"
            );
            "[]".to_string()
        }
    };

    let code = format!(
        "use crate::models::Contributor;\npub const CONTRIBUTORS: &[Contributor] = &{list};\n"
    );
    fs::write("src/extras/contributors.rs", code).unwrap();
}

fn fetch() -> Result<Vec<Contributor>, Box<dyn Error>> {
    let token = env::var("COLLABORATORS_API_TOKEN").or_else(|_| env::var("GITHUB_TOKEN"))?;
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {token}").parse()?);
    let client = Client::builder()
        .user_agent("RustLangES")
        .default_headers(headers)
        .build()?;

    let mut commits: HashMap<String, u32> = HashMap::new();
    let repos: Vec<Repo> = get_all(
        &client,
        "https://api.github.com/orgs/RustLangES/repos?type=public&per_page=100",
    )?;

    for repo in repos {
        // Los forks tienen el historial de otros proyectos
        if repo.fork {
            continue;
        }

        let mut upstream = Vec::new();
        if let Some((_, original)) = UPSTREAMS.iter().find(|(name, _)| *name == repo.name) {
            upstream = get_contributors(&client, original)?;
        }

        for c in get_contributors(&client, &repo.full_name)? {
            let is_upstream = upstream.iter().any(|u| u.login == c.login);
            let is_excluded = NOT_COMMUNITY_MEMBERS.contains(&c.login.as_str());

            if c.kind == "User" && !is_upstream && !is_excluded {
                *commits.entry(c.login).or_insert(0) += c.contributions;
            }
        }
    }

    let mut contributors = Vec::new();
    for (login, contributions) in commits {
        let url = format!("https://api.github.com/users/{login}");
        let mut contributor: Contributor = client.get(url).send()?.error_for_status()?.json()?;
        contributor.contributions = contributions;
        contributor.avatar.push_str("&s=128");
        contributors.push(contributor);
    }

    contributors.sort_by_key(|c| (Reverse(c.contributions), c.login.to_lowercase()));
    Ok(contributors)
}

fn get_contributors(client: &Client, repo: &str) -> Result<Vec<RepoContributor>, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{repo}/contributors?per_page=100");
    get_all(client, &url)
}

fn get_all<T: DeserializeOwned>(client: &Client, url: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let mut items = Vec::new();

    for page in 1.. {
        let response = client
            .get(format!("{url}&page={page}"))
            .send()?
            .error_for_status()?;

        if response.status() == StatusCode::NO_CONTENT {
            break;
        }

        let page_items: Vec<T> = response.json()?;
        let is_last_page = page_items.len() < 100;
        items.extend(page_items);

        if is_last_page {
            break;
        }
    }

    Ok(items)
}
