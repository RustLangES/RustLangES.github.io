use leptos::*;
use leptos_router::*;

use crate::{components::{CommunityCard, NextIcon}, extras::OTHER_COMUNITIES};

#[component]
pub fn OtherCommunities(#[prop(default = false)] main: bool) -> impl IntoView {
    view! {
        <section class="bg-orange-200 py-20">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl text-left mb-6">
                    <span class="font-work-sans font-light">"Otras "</span>
                    <span class="font-alfa-slab text-orange-500">"Comunidades"</span>
                    <span class="font-work-sans font-light">" recomendadas "</span>
                </h2>
                <div class="w-full grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-8 gap-y-8">
                    {OTHER_COMUNITIES
                        .iter()
                        .take(if main { 5 } else { OTHER_COMUNITIES.len()  })
                        .map(|item| {
                            view! {
                                <CommunityCard
                                    name=item.name
                                    description=item.description
                                    link=item.link
                                    icon=item.icon
                                    brand_src=item.brand_src
                                    brand_alt=item.brand_alt
                                />
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
                <div class="w-full flex justify-end my-3">
                    <A href="/comunidad" class="text-black/80 hover:text-orange-500 fill-black/80 hover:fill-orange-500 font-work-sans font-light text-2xl flex justify-center items-center">
                        Ver todas las comunidades
                        <span class="inline-block ml-2">
                            <NextIcon class="fill-current" size=20 />
                        </span>
                    </A>
                </div>
            </div>
        </section>
    }
}
