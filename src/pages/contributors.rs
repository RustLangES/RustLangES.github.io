use std::collections::HashMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::{
    contributor_card::{ContributorCard, ContributorInfo},
    footer::Footer,
};

const GRAPH_QUERY: &str = r#"query OrganizationContributors {
  organization(login: "RustLangES") {
    repositories(first: 100) {
      nodes {
        name
        collaborators(first: 100) {
          nodes {
            login
            avatarUrl
            url
            bio
            twitterUsername
            location
            contributionsCollection(organizationID: "O_kgDOBHON2w") {
              totalCommitContributions
              totalPullRequestContributions
              totalIssueContributions
              totalRepositoryContributions
            }
          }
        }
      }
    }
  }
}
"#;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    login: String,
    avatar_url: String,
    url: String,
    bio: Option<String>,
    twitter_username: Option<String>,
    location: Option<String>,
    contributions_collection: Option<ContributionCollection>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionCollection {
    #[serde(rename = "totalCommitContributions")]
    commits: u64,
    #[serde(rename = "totalPullRequestContributions")]
    pull_request: u64,
    #[serde(rename = "totalIssueContributions")]
    issues: u64,
    #[serde(rename = "totalRepositoryContributions")]
    repository: u64,
}

impl ContributionCollection {
    fn total(&self) -> u64 {
        self.commits + self.issues + self.pull_request + self.repository
    }
}

#[derive(Serialize)]
struct GqlBody<'a> {
    query: &'a str,
}

#[cfg(feature = "ssr")]
async fn fetch_contributors_impl() -> Vec<Contributor> {
    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        "User-Agent",
        "RustLangES-Website/1.0".parse().unwrap(),
    );
    if !token.is_empty() {
        if let Ok(auth_value) = format!("Bearer {}", token).parse() {
            headers.append("Authorization", auth_value);
        }
    }

    let client = match reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            leptos::logging::log!("Failed to build HTTP client: {e:?}");
            return vec![];
        }
    };

    let text = match client
        .post("https://api.github.com/graphql")
        .json(&GqlBody { query: GRAPH_QUERY })
        .send()
        .await
    {
        Ok(r) => match r.text().await {
            Ok(t) => t,
            Err(e) => {
                leptos::logging::log!("Error reading response body: {e:?}");
                return vec![];
            }
        },
        Err(e) => {
            leptos::logging::log!("Error sending GitHub request: {e:?}");
            return vec![];
        }
    };

    leptos::logging::log!("GitHub API response: {text:?}");

    let json: leptos::serde_json::Value = match leptos::serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            leptos::logging::log!("Error parsing JSON: {e:?}");
            return vec![];
        }
    };

    let empty = vec![];
    let nodes = json["data"]["organization"]["repositories"]["nodes"]
        .as_array()
        .unwrap_or(&empty);

    let mut contributor_map: HashMap<String, Contributor> = HashMap::new();

    for repo_node in nodes {
        if repo_node["collaborators"].is_null() {
            continue;
        }
        let Some(collaborators) = repo_node["collaborators"]["nodes"].as_array() else {
            continue;
        };
        for collab_json in collaborators {
            let Ok(c) = leptos::serde_json::from_value::<Contributor>(collab_json.clone()) else {
                continue;
            };
            contributor_map
                .entry(c.login.clone())
                .and_modify(|o| {
                    if let (Some(o_cc), Some(c_cc)) = (
                        o.contributions_collection.as_mut(),
                        c.contributions_collection.as_ref(),
                    ) {
                        if c_cc.total() > o_cc.total() {
                            o_cc.commits = c_cc.commits;
                            o_cc.pull_request = c_cc.pull_request;
                            o_cc.issues = c_cc.issues;
                            o_cc.repository = c_cc.repository;
                        }
                    }
                })
                .or_insert(c);
        }
    }

    let mut result: Vec<Contributor> = contributor_map
        .into_values()
        .filter(|c| {
            c.contributions_collection
                .as_ref()
                .is_some_and(|cc| cc.total() > 0)
        })
        .collect();

    result.sort_by_key(|c| {
        c.contributions_collection
            .as_ref()
            .map(|cc| cc.total())
            .unwrap_or(0)
    });
    result.reverse();
    result
}

pub async fn fetch_contributors() -> Vec<Contributor> {
    #[cfg(feature = "ssr")]
    {
        return tokio::task::spawn_local(fetch_contributors_impl())
            .await
            .unwrap_or_default();
    }
    #[cfg(not(feature = "ssr"))]
    {
        vec![]
    }
}

#[component]
pub fn Contributors() -> impl IntoView {
    let contributors = Resource::new_blocking(|| (), |_| async { fetch_contributors().await });

    view! {
        // Hero
        <div class="w-full min-h-[45dvh] rustlang-es-background-secondary dark:bg-[#3E1C96CC] text-akira flex items-center justify-center py-16">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col items-center justify-center gap-6 text-center">
                <h1 class="uppercase leading-tight">"Colaboradores"</h1>
                <p class="text-base font-normal font-body max-w-lg leading-relaxed">
                    "Personas que hacen posible RustLangES con su código, traducciones y contenido."
                </p>
            </div>
        </div>

        // Contributors grid
        <section class="bg-white dark:bg-neutral-900 py-20 w-full">
            <div class="container max-w-7xl mx-auto px-6">
                <Suspense fallback=move || view! {
                    <div class="flex items-center justify-center py-20">
                        <p class="text-neutral-500 font-body">"Cargando colaboradores..."</p>
                    </div>
                }>
                    {move || contributors.get().map(|contribs| {
                        if contribs.is_empty() {
                            view! {
                                <div class="flex flex-col items-center justify-center py-20 gap-4">
                                    <p class="text-neutral-500 font-body text-lg">
                                        "No se encontraron colaboradores."
                                    </p>
                                    <p class="text-neutral-400 font-body text-sm">
                                        "Asegúrate de configurar la variable de entorno GITHUB_TOKEN."
                                    </p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 w-full">
                                    {contribs.into_iter().map(|c| {
                                        let total = c.contributions_collection
                                            .as_ref()
                                            .map(|cc| cc.total())
                                            .unwrap_or(0);
                                        let info = ContributorInfo {
                                            login: c.login,
                                            avatar_url: c.avatar_url,
                                            url: c.url,
                                            twitter_username: c.twitter_username,
                                            location: c.location,
                                            total_contributions: total,
                                        };
                                        view! { <ContributorCard contributor=info /> }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>
            </div>
        </section>

        <Footer />
    }
}
