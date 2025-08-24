use std::collections::HashMap;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

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

pub async fn fetch_contributors() -> Vec<Contributor> {
    // let request_body = json!({
    //     "query": GRAPH_QUERY,
    // });

    let mut headers = reqwest::header::HeaderMap::new();

    headers.append("User-Agent", "RustLangES Automation Agent".parse().unwrap());
    headers.append("Authorization", "Bearer {}".parse().unwrap());

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = client
        .post("https://api.github.com/graphql")
        // .json(&request_body)
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
        .filter_map(|repo| {
            (!repo["collaborators"].is_null())
                .then(|| repo["collaborators"]["nodes"].as_array().unwrap())
        })
        .flatten()
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
        <section>
        </section>
    }
}
