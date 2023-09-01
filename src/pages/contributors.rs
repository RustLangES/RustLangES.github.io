use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};

use crate::components::{cards::contributor_card::ContributorCard, footer::Footer, header::Header};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contributor {
    login: String,
    avatar_url: String,
    url: String,
}

async fn fetch_contributors() -> Result<Vec<Contributor>> {
    let response = reqwasm::http::Request::get(
        &"https://api.github.com/repos/RustLangES/rustlanges.github.io/contributors",
    )
    .send()
    .await?
    .json::<Vec<Contributor>>()
    .await?
    .into_iter()
    .map(|item| item)
    .collect::<Vec<_>>();
    Ok(response)
}

#[component]
pub fn Contributors() -> impl IntoView {
    let contributors_results = create_local_resource(move || (), |_| fetch_contributors());

    let contributors_view = move || {
        let data = contributors_results.read();
        let Some(Ok(items)) = data else { return None };
        let result = items
            .iter()
            .map(|item| {
                view! {
                    <ContributorCard
                        name=item.login.clone()
                        description=""
                        link=item.url.clone()
                        brand_src=item.avatar_url.clone()
                    />
                }
            })
            .collect::<Fragment>();
        Some(result.into_view())
    };

    view! {
        <div>
            <Header/>
            <main>
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
            </main>
            <Footer/>
        </div>
    }
}
