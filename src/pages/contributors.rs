use futures::future::join_all;
use leptos::{
    create_local_resource, error::Result, island, serde, view, Fragment, IntoView, SignalGet,
};
use serde::{Deserialize, Serialize};

use crate::components::ContributorCard;
use crate::{error, log};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contributor {
    login: String,
    avatar_url: String,
    html_url: String,
    bio: Option<String>,
    twitter_username: Option<String>,
    location: Option<String>,
    contributions: Option<i32>,
}

async fn fetch_contributors() -> Result<Vec<Contributor>> {
    let response = reqwasm::http::Request::get(
        "https://api.github.com/repos/RustLangES/rustlanges.github.io/contributors",
    )
    .send()
    .await?
    .json::<Vec<Contributor>>()
    .await?;

    let response = join_all(response.iter().map(|c| fetch_contributor_info(c)))
        .await
        .iter()
        .flat_map(|c| c.clone().ok())
        .collect::<Vec<Contributor>>();

    Ok(response)
}

async fn fetch_contributor_info(contributor: &Contributor) -> Result<Contributor> {
    Ok(reqwasm::http::Request::get(&format!(
        "https://api.github.com/users/{}",
        contributor.login
    ))
    .send()
    .await?
    .json::<Contributor>()
    .await
    .inspect(|c| log!("{c:?} - {contributor:?}"))
    .inspect_err(|e| error!("Error: {e:?}"))
    .map(|c| Contributor {
        contributions: contributor.contributions,
        ..c
    })?)
}

#[island]
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
        <section class="bg-orange-300/30 py-16">
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
