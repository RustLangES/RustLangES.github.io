use leptos::{prelude::*, serde_json};

#[cfg(feature = "ssr")]
use crate::components::ContributorCard;
#[cfg(feature = "ssr")]
use leptos::serde_json::json;
use leptos::{component, view, IntoView};
#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use std::collections::HashMap;

/// Test this query on: https://docs.github.com/es/graphql/overview/explorer
#[cfg(feature = "ssr")]
const GRAPH_QUERY: &str = r#"
query OrganizationContributors($login: String!, $first: Int!, $after: String, $collaboratorsFirst: Int!, $collaboratorsAfter: String) {
  organization(login: $login) {
    repositories(first: $first, after: $after) {
      nodes {
        name
        collaborators(first: $collaboratorsFirst, after: $collaboratorsAfter) {
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
          pageInfo {
            hasNextPage
            endCursor
          }
        }
      }
      pageInfo {
        hasNextPage
        endCursor
      }
    }
  }
}
"#;

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
pub struct GithubResponse {
    pub data: Option<OrganizationData>,
}

#[cfg(feature = "ssr")]
fn empty_org_data() -> OrganizationData {
    OrganizationData {
        organization: OrganizationRepositories {
            repositories: RepositoryConnection {
                nodes: Some(Vec::new()),
                page_info: PageInfo {
                    has_next_page: false,
                    has_previous_page: false,
                    start_cursor: None,
                    end_cursor: None,
                },
            },
        },
    }
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    #[serde(default)]
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryConnection {
    #[serde(default)]
    pub nodes: Option<Vec<Repository>>,
    pub page_info: PageInfo,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationData {
    pub organization: OrganizationRepositories,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationRepositories {
    pub repositories: RepositoryConnection,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaboratorConnection {
    #[serde(default)]
    pub nodes: Option<Vec<Contributor>>,
    pub page_info: PageInfo,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    name: String,
    collaborators: Option<CollaboratorConnection>,
}

#[cfg(feature = "ssr")]
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

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionCollection {
    #[serde(rename = "totalCommitContributions")]
    commits: Option<u64>,
    #[serde(rename = "totalPullRequestContributions")]
    pull_request: Option<u64>,
    #[serde(rename = "totalIssueContributions")]
    issues: Option<u64>,
    #[serde(rename = "totalRepositoryContributions")]
    repository: Option<u64>,
    #[serde(skip)]
    total: u64,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributorsResponse {
    pub contributors: Vec<Contributor>,
    pub total: usize,
}

#[cfg(feature = "ssr")]
mod config {
    pub const ORGANIZATION_LOGIN: &str = "RustLangES";
    pub const REPO_PAGE_SIZE: i32 = 4;
    pub const COLLAB_PAGE_SIZE: i32 = 40;
    pub const API_DELAY_MS: u64 = 200;
}

#[cfg(feature = "ssr")]
pub async fn fetch_contributors() -> ContributorsResponse {
    println!("DEBUG: Starting fetch_contributors()");
    let mut all_contributors = fetch_all_contributors_with_pagination().await;
    println!(
        "DEBUG: fetch_contributors returned {} contributors",
        all_contributors.len()
    );

    all_contributors.sort_by_key(|a| {
        a.contributions_collection
            .as_ref()
            .map(|c| c.total)
            .unwrap_or(1)
    });
    all_contributors.reverse();

    let total = all_contributors.len();

    ContributorsResponse {
        contributors: all_contributors,
        total,
    }
}

#[cfg(feature = "ssr")]
async fn fetch_all_contributors_with_pagination() -> Vec<Contributor> {
    let mut repo_cursor: Option<String> = None;
    let mut has_next_repo = true;

    let mut seen: HashMap<String, Contributor> = HashMap::new();

    while has_next_repo {
        let collab_cursor: Option<String> = None;

        let response = execute_repository_query(
            config::ORGANIZATION_LOGIN,
            config::REPO_PAGE_SIZE,
            repo_cursor.clone(),
            config::COLLAB_PAGE_SIZE,
            collab_cursor.clone(),
        )
        .await;

        println!(
            "Fetched a page of repositories with {} nodes",
            response
                .organization
                .repositories
                .nodes
                .as_ref()
                .map(|n| n.len())
                .unwrap_or(0)
        );

        let repos = response.organization.repositories;

        for repo in repos.nodes.unwrap_or_default() {
            if let Some(collabs) = repo.collaborators {
                for collaborator in collabs.nodes.unwrap_or_default() {
                    process_contributor(&mut seen, collaborator);
                }

                let mut collab_page_info = collabs.page_info;
                let repo_name = repo.name.clone();
                while collab_page_info.has_next_page {
                    let collab_cursor = collab_page_info.end_cursor.clone();

                    println!(
                        "DEBUG: Fetching more collabs for repo '{}', repo_cursor={:?}, collab_cursor={:?}",
                        repo_name, repo_cursor, collab_cursor
                    );

                    let collab_response = execute_repository_query(
                        config::ORGANIZATION_LOGIN,
                        1,
                        None, // FIX: Use None to stay on the same repo, not the repo cursor
                        config::COLLAB_PAGE_SIZE,
                        collab_cursor,
                    )
                    .await;

                    if let Some(collab_repo) = collab_response
                        .organization
                        .repositories
                        .nodes
                        .and_then(|n| n.into_iter().next())
                    {
                        if let Some(next_collabs) = collab_repo.collaborators {
                            for collaborator in next_collabs.nodes.unwrap_or_default() {
                                process_contributor(&mut seen, collaborator);
                            }
                            collab_page_info = next_collabs.page_info;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                    tokio::time::sleep(tokio::time::Duration::from_millis(config::API_DELAY_MS))
                        .await;
                }
            }
        }

        has_next_repo = repos.page_info.has_next_page;
        repo_cursor = repos.page_info.end_cursor;

        tokio::time::sleep(tokio::time::Duration::from_millis(config::API_DELAY_MS)).await;
    }

    seen.into_values().collect()
}

#[cfg(feature = "ssr")]
fn process_contributor(seen: &mut HashMap<String, Contributor>, mut contributor: Contributor) {
    if let Some(cc) = contributor.contributions_collection.as_mut() {
        cc.total = cc.commits.unwrap_or(0)
            + cc.issues.unwrap_or(0)
            + cc.pull_request.unwrap_or(0)
            + cc.repository.unwrap_or(0);
        if cc.total == 0 {
            cc.total = 1;
        }
    }

    let login = contributor.login.clone();
    seen.entry(login)
        .and_modify(|existing| {
            if let (Some(o), Some(n)) = (
                existing.contributions_collection.as_mut(),
                contributor.contributions_collection.as_ref(),
            ) {
                o.total = o.total.max(n.total);
            }
        })
        .or_insert(contributor);
}

#[cfg(feature = "ssr")]
async fn execute_repository_query(
    login: &str,
    first: i32,
    after: Option<String>,
    collaborators_first: i32,
    collaborators_after: Option<String>,
) -> OrganizationData {
    let token = std::env::var("COLLABORATORS_API_TOKEN").unwrap_or_default();
    if token.is_empty() {
        leptos::logging::warn!("COLLABORATORS_API_TOKEN is empty or not set!");
        return empty_org_data();
    }

    let request_body = json!({
        "query": GRAPH_QUERY,
        "variables": {
            "login": login,
            "first": first,
            "after": after,
            "collaboratorsFirst": collaborators_first,
            "collaboratorsAfter": collaborators_after,
        },
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.github.com/graphql")
        .header("User-Agent", "RustLangES Automation Agent")
        .header("Authorization", format!("Bearer {}", token))
        .json(&request_body)
        .send()
        .await;

    let text = match res {
        Ok(r) => r.text().await.unwrap_or_default(),
        Err(e) => {
            leptos::logging::error!("Request failed: {:?}", e);
            return empty_org_data();
        }
    };

    let parsed: GithubResponse = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            leptos::logging::error!("JSON Error: {} en col {}", e, e.column());
            return empty_org_data();
        }
    };

    let org_data = parsed.data.unwrap_or_else(empty_org_data);
    let repo_count = org_data
        .organization
        .repositories
        .nodes
        .as_ref()
        .map(|n| n.len())
        .unwrap_or(0);
    println!("DEBUG: Parsed org data, repos count: {}", repo_count);

    org_data
}

#[component]
pub fn Contributors() -> impl IntoView {
    {
        view! {
            <section class="bg-orange-300/30 dark:bg-transparent py-16 min-h-[80vh]">
                <div class="flex flex-col gap-y-6 container mx-auto px-4">
                    <h2 class="text-3xl text-left mb-3">
                        <span class="font-work-sans font-light">"Nuestros "</span>
                        <span class="font-alfa-slab text-orange-500">"Colaboradores"</span>
                    </h2>
                    <p class="md:max-w-[800px] mb-2">
                        Gracias al esfuerzo y dedicación de estos extraordinarios colaboradores open source, los servicios y páginas de nuestra comunidad se mantienen activos y en constante evolución. Su pasión por el código abierto y el desarrollo de Rust es el corazón que impulsa nuestro crecimiento.
                    </p>
                    <p class="md:max-w-[800px] mb-2">
                        "Te invitamos a unirte a esta vibrante comunidad, explorar nuestros repositorios en "
                        <a href="https://github.com/RustLangES" class="underline" target="_blank">
                            GitHub
                        </a> " y contribuir con tu talento."
                    </p>
                    <p class="md:max-w-[800px]">
                        <strong>Juntos</strong>
                        , podemos seguir construyendo un ecosistema Rust más fuerte y accesible para todos.
                    </p>
                    <div class="mt-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-5 gap-6">
                        {
                            #[cfg(feature = "ssr")]
                            view! {
                                <Await future=fetch_contributors() let:res>
                                    {res
                                        .contributors
                                        .iter()
                                        .map(|item| {
                                            view! {
                                                <ContributorCard
                                                    name=item.login.clone()
                                                    description=item.bio.clone()
                                                    link=item.url.clone()
                                                    brand_src=item.avatar_url.clone()
                                                    twitter=item.twitter_username.clone()
                                                    location=item.location.clone()
                                                    contributions=item
                                                        .contributions_collection
                                                        .as_ref()
                                                        .map(|c| c.total)
                                                        .unwrap_or(1)
                                                />
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </Await>
                            }
                        }
                    </div>
                </div>
            </section>
        }
    }
}
