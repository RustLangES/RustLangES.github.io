use leptos::*;

use crate::{components::cards::community_card::CommunityCard, extras::OTHER_COMUNITIES};

#[component]
pub fn OtherCommunities() -> impl IntoView {
    view! {
        <section class="bg-orange-100 py-20">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl text-left mb-6">
                    <span class="font-work-sans font-light">"Otras "</span>
                    <span class="font-alfa-slab text-orange-500">"Comunidades"</span>
                    <span class="font-work-sans font-light">" recomendadas "</span>
                </h2>
                <div class="w-full grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-8 gap-y-8">
                    {OTHER_COMUNITIES
                        .into_iter()
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
            </div>
        </section>
    }
}
