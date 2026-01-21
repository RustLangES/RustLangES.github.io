use leptos::{component, view, IntoView};
use leptos::prelude::*;

use crate::{
    components::{CommunityCard, NextIcon},
    extras::{OTHER_COMMUNITIES, RUST_COMMUNITIES},
};

#[component]
pub fn OtherCommunities(
    #[prop(default = false)] show_more: bool,
    #[prop(default = false)] other_communities: bool,
) -> impl IntoView {
    let communities = match (other_communities, show_more) {
        (true, false) => OTHER_COMMUNITIES.to_vec(),
        (false, false) => RUST_COMMUNITIES.to_vec(),
        (_, true) => RUST_COMMUNITIES.to_vec(),
    };

    view! {
        <section class="bg-orange-50 dark:bg-transparent py-20">
            <div class="container mx-auto px-4">

                {if other_communities {
                    view! {
                        <h2 class="text-3xl text-left mb-6">
                            <span class="font-work-sans font-light">"Otras "</span>
                            <span class="font-alfa-slab text-orange-500">"Comunidades"</span>
                            <span class="font-work-sans font-light">" recomendadas "</span>
                        </h2>
                    }
                        .into_any()
                } else {
                    view! {
                        <h2 class="text-3xl text-left mb-6">
                            <span class="font-work-sans font-light">"Comunidades de"</span>
                            <span class="font-alfa-slab text-orange-500">" Rust"</span>
                        </h2>
                    }
                        .into_any()
                }}
                <div class="w-full grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-8 gap-y-8">

                    {communities
                        .iter()
                        .take(if show_more { 5 } else { communities.len() })
                        .map(|item| {
                            let image_src = if cfg!(debug_assertions)
                                && item.brand_src.starts_with("/gen_assets")
                            {
                                format!("/assets{}", item.brand_src)
                            } else {
                                item.brand_src.to_string()
                            };
                            view! {
                                <CommunityCard
                                    name=item.name
                                    description=item.description
                                    link=item.link
                                    icon=item.icon
                                    brand_src=image_src
                                    brand_alt=item.brand_alt
                                />
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
                {if show_more {
                    view! {
                        <div class="w-full flex justify-end my-3">
                            <a
                                href="/comunidades"
                                class="text-black/80 dark:text-white/80 hover:text-orange-500 fill-black/80  dark:fill-white/80 hover:fill-orange-500 font-work-sans font-light text-2xl flex justify-center items-center"
                            >
                                "Ver todas las comunidades"
                                <span class="inline-block ml-2">
                                    <NextIcon class="fill-current" size=20 />
                                </span>
                            </a>
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}

            </div>
        </section>
    }
}
