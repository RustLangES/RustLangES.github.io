use leptos::{component, view, Fragment, IntoView};
#[cfg(not(debug_assertions))]
use leptos::{create_local_resource, error::Result, serde_json::json, SignalGet};
use serde::{Deserialize, Serialize};

use crate::components::ContributorCard;

#[cfg(not(debug_assertions))]
const GRAPH_QUERY: &str = r#"
query OrganizationContributors {
  organization(login: "RustLangES") {
    repositories(first: 100) {
      nodes {
        collaborators(first: 100) {
          nodes {
            login
            avatarUrl
            url
            bio
            twitterUsername
            location
            contributionsCollection {
              totalCommitContributions
            }
          }
        }
      }
    }
  }
}
"#;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contributor {
    login: String,
    avatar_url: String,
    url: String,
    bio: Option<String>,
    twitter_username: Option<String>,
    location: Option<String>,
    contributions_collection: ContributionCollection,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributionCollection {
    total_commit_contributions: u64,
}

#[cfg(not(debug_assertions))]
async fn fetch_contributors() -> Result<Vec<Contributor>> {
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
        .send()
        .await?
        .json()
        .await?;

    let mut res = res["data"]["organization"]["repositories"]["nodes"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .flat_map(|repo| repo["contributors"]["nodes"].as_array().unwrap())
        .map(|c| leptos::serde_json::from_value::<Contributor>(c.clone()).map_err(|e| e.into()))
        .collect::<Result<Vec<Contributor>>>()?;

    res.sort_by_key(|a| a.contributions_collection.total_commit_contributions);

    Ok(res)
}

#[cfg_attr(not(debug_assertions), component)]
#[cfg(not(debug_assertions))]
pub fn Contributors() -> impl IntoView {
    let contributors_results = create_local_resource(move || (), |()| fetch_contributors());
    let contributorMapper = |item: &Contributor| {
        view! {
            <ContributorCard
                name=item.login.clone()
                description=item.bio.clone()
                link=item.html_url.clone()
                brand_src=item.avatar_url.clone()
                twitter=item.twitter_username.clone()
                location=item.location.clone()
                contributions=item.contributions.unwrap_or(1)
            />
        }
    };

    let contributors_view = move || {
        let contributors = contributors_results.get()?.ok()?;
        let result = contributors
            .iter()
            .map(contributorMapper)
            .collect::<Fragment>();
        Some(result.into_view())
    };

    view! {
        <section class="bg-orange-300/30 dark:bg-transparent py-16">
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

#[cfg_attr(debug_assertions, component)]
#[cfg(debug_assertions)]
pub fn Contributors() -> impl IntoView {
    let contributors = [Contributor {
        login: "Phosphorus-M".to_owned(),
        avatar_url: "https://avatars.githubusercontent.com/u/19656993?v=4".to_owned(),
        url: "https://github.com/Phosphorus-M".to_owned(),
        bio: None,
        twitter_username: Some("Phosphorus_M".to_owned()),
        location: Some("Argentina".to_owned()),
        contributions_collection: ContributionCollection {
            total_commit_contributions: 499,
        },
    },
    Contributor {
        login: "SergioRibera".to_owned(),
        avatar_url: "https://avatars.githubusercontent.com/u/56278796?u=9e3dac947b4fd3ca2f1a05024e083c64e4c69cfe&v=4".to_owned(),
        url: "https://github.com/SergioRibera".to_owned(),
        bio: Some("22yo Rustacean and Open Source lover\r\nI teach, Promote and Give Technical talks of rust with @RustLangES".to_owned()),
        twitter_username: Some("sergioribera_rs".to_owned()),
        location: Some("Santa Cruz de la Sierra, Bolivia".to_owned()),
        contributions_collection: ContributionCollection {
          total_commit_contributions: 2015
        }
    }];
    let contributorMapper = |item: &Contributor| {
        view! {
            <ContributorCard
                name=item.login.clone()
                description=item.bio.clone()
                link=item.url.clone()
                brand_src=item.avatar_url.clone()
                twitter=item.twitter_username.clone()
                location=item.location.clone()
                contributions=item.contributions_collection.total_commit_contributions
            />
        }
    };

    let contributors_view = move || {
        let result = contributors
            .iter()
            .map(contributorMapper)
            .collect::<Fragment>();
        Some(result.into_view())
    };

    view! {
        <section class="bg-orange-300/30 dark:bg-transparent py-16">
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
