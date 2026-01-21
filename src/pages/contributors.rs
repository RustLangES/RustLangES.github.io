use std::collections::HashMap;
use leptos::prelude::*;

use leptos::{component, serde_json::json, view, IntoView};
use serde::{Deserialize, Serialize};

use crate::components::ContributorCard;

// Test this query on: https://docs.github.com/es/graphql/overview/explorer
const GRAPH_QUERY: &str = r#"
query OrganizationContributors {
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
    #[serde(skip)]
    total: u64,
}

#[cfg(feature = "ssr")]
pub async fn fetch_contributors() -> Vec<Contributor> {
    let request_body = json!({
        "query": GRAPH_QUERY,
    });

    let mut headers = reqwest::header::HeaderMap::new();

    headers.append("User-Agent", "RustLangES Automation Agent".parse().unwrap());
    headers.append(
        "Authorization",
        format!("Bearer {}", env!("COLLABORATORS_API_TOKEN"))
            .parse()
            .unwrap(),
    );

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = client
        .post("https://api.github.com/graphql")
        .json(&request_body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    leptos::logging::log!("Raw: {res:?}");

    let res: leptos::serde_json::Value = leptos::serde_json::from_str(&res).unwrap();

    let mut res = res["data"]["organization"]["repositories"]["nodes"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter(|&repo| !repo["collaborators"].is_null())
        .flat_map(|repo| repo["collaborators"]["nodes"].as_array().unwrap())
        .filter_map(|c| leptos::serde_json::from_value::<Contributor>(c.clone()).ok())
        .fold(HashMap::new(), |prev, c| {
            let mut prev = prev;
            prev.entry(c.login.clone())
                .and_modify(|o: &mut Contributor| {
                    match (
                        o.contributions_collection.as_mut(),
                        c.contributions_collection.as_ref(),
                    ) {
                        (Some(o), Some(c)) => {
                            o.total = o.total.max(
                                (o.commits + o.issues + o.pull_request + o.repository)
                                    .max(c.commits + c.issues + c.pull_request + c.repository),
                            )
                        }
                        (Some(o), None) => o.total = 1,
                        _ => {}
                    }
                })
                .or_insert(c);
            prev
        })
        .into_values()
        .filter(|c| {
            c.contributions_collection
                .as_ref()
                .is_some_and(|cc| cc.total != 0)
        })
        .collect::<Vec<_>>();

    res.sort_by_key(|a| {
        a.contributions_collection
            .as_ref()
            .map(|c| c.total)
            .unwrap_or(1)
    });

    res.reverse();

    res
}

#[component]
pub fn Contributors() -> impl IntoView {
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
                            <Await future=fetch_contributors() let:contributors>
                                {contributors
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
