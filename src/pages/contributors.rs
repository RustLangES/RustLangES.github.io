use std::collections::HashMap;

use leptos::{
    component, create_local_resource, error::Result, island, serde_json::json, view, Fragment,
    IntoView, SignalGet,
};
use serde::{Deserialize, Serialize};

use crate::components::ContributorCard;

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
}

pub async fn fetch_contributors() -> Result<Vec<Contributor>> {
    let request_body = json!({
        "query": GRAPH_QUERY,
    });

    let mut headers = reqwest::header::HeaderMap::new();

    headers.append(
        "Authorization",
        format!("Bearer {}", env!("GITHUB_API_TOKEN"))
            .parse()
            .unwrap(),
    );

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;

    let res: leptos::serde_json::Value = client
        .post("https://api.github.com/graphql")
        .json(&request_body)
        .send()
        .await?
        .json()
        .await?;

    let mut res = res["data"]["organization"]["repositories"]["nodes"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
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
                        (Some(o), Some(c)) => o.commits += c.commits,
                        (Some(o), None) => o.commits += 1,
                        _ => {}
                    }
                })
                .or_insert(c);
            prev
        })
        .into_values()
        .collect::<Vec<_>>();

    res.sort_by_key(|a| {
        a.contributions_collection
            .as_ref()
            .map(|c| c.repository)
            .unwrap_or(1)
    });

    res.reverse();

    Ok(res)
}

#[island]
pub fn Contributors() -> impl IntoView {
    let contributors = create_local_resource(move || (), |()| fetch_contributors());

    let contributorMapper = |item: &Contributor| {
        view! {
            <ContributorCard
                name=item.login.clone()
                description=item.bio.clone()
                link=item.url.clone()
                brand_src=item.avatar_url.clone()
                twitter=item.twitter_username.clone()
                location=item.location.clone()
                contributions=item.contributions_collection.as_ref().map(|c| c.commits).unwrap_or(1)
            />
        }
    };

    let contributors_view = move || {
        let result = contributors
            .get()?
            .ok()?
            .iter()
            .map(contributorMapper)
            .collect::<Fragment>();
        Some(result.into_view())
    };

    view! {
        <section class="bg-orange-300/30 dark:bg-transparent py-16 min-h-[80vh]">
            <div class="flex flex-col gap-y-6 container mx-auto px-4">
                <h2 class="text-3xl text-left mb-6">
                    <span class="font-work-sans font-light">"Nuestros "</span>
                    <span class="font-alfa-slab text-orange-500">"Colaboradores"</span>
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-5 gap-6">
                    {contributors_view}
                </div>
            </div>
        </section>
    }
}
