use leptos::prelude::*;
use rustlanges_components::input::InputSearch;

use crate::{
    components::community_card::CommunityCard,
    extras::{OTHER_COMMUNITIES, RUST_COMMUNITIES},
};

#[allow(non_snake_case)]
#[island]
pub fn OtherCommunities() -> impl IntoView {
    let query = RwSignal::new("".to_string());
    let results = Signal::derive(move || {
        let query = query.read();
        RUST_COMMUNITIES
            .iter()
            .chain(OTHER_COMMUNITIES)
            .filter(|community| community.matches(&query))
            .collect::<Vec<_>>()
    });

    view! {
        <section class="bg-orange-100 dark:bg-neutral-950 p-4 lg:p-20 w-full">
            <div class="flex flex-col justify-center items-center w-full container xl:max-w-7xl lg:px-36 lg:pb-5 mx-auto">
                <h2 class="text-h2 text-center mb-4">"Otras Comunidades Rust"</h2>
                <div>
                    <p class="text-center max-w-lg">"Encuentra la comunidad más cercana a ti"</p>
                </div>
                <div class="flex flex-col justify-center mb-8 container mt-10 mx-auto max-w-fit">
                    <InputSearch on_input=move |text| query.set(text) />
                </div>
            </div>
            <div class="flex flex-row justify-center items-center gap-4 max-w-full m-auto">
                <div class="items-center gap-8 overflow-x-auto p-4 grid md:grid-cols-2 xl:grid-cols-3">
                    <For each=move || results.get() key=|community| community.link let:community>
                        <CommunityCard community />
                    </For>
                </div>
            </div>

            <Show when=move || results.read().is_empty()>
                <p class="text-paragraph-1 text-center py-3">"No se encontraron comunidades"</p>
            </Show>
        </section>
    }
}
