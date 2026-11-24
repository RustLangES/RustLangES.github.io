use leptos::prelude::*;
use rustlanges_components::{icons::IconVariant, input::InputSearch};

use crate::{components::community_card::CommunityCard, models::Community};

const COMMUNITIES: &[Community] = &[
    Community {
        name: "Rust Colombia",
        country: "Colombia",
        description: "Comunidad de Rust con dos sedes, en Medellín y en Bogotá",
        logo: "https://rustlang-es.org/gen_assets/rust-colombia.webp",
        link: Some(("https://discord.gg/krm22PuDVw", IconVariant::Discord)),
    },
    Community {
        name: "Rust Venezuela",
        country: "Venezuela duh",
        description: "Venezuela dih",
        logo: "https://rustlang-es.org/gen_assets/rust-venezuela.webp",
        link: Some(("https://t.me/rustlangVE", IconVariant::Telegram)),
    },
    Community {
        name: "Rust Argentina",
        country: "Argentina claro",
        description: "Anteriormente Rust Argentina, hoy una comunidad internacional",
        logo: "https://rustlang-es.org/gen_assets/rust-argentina.webp",
        link: Some(("https://t.me/rust_lang_es", IconVariant::Telegram)),
    },
    Community {
        name: "Rust Ecuador",
        country: "Ecuador",
        description: "¡Comunidad de Rust en Ecuador!",
        logo: "https://rustlang-es.org/gen_assets/rust_ecuador.webp",
        link: Some(("https://t.me/rustecuador", IconVariant::Telegram)),
    },
    Community {
        name: "Rust MX",
        country: "Mexico",
        description: "¡Comunidad de Rust en Mexico!",
        logo: "https://rustlang-es.org/gen_assets/RustMX-min.webp",
        link: Some(("https://t.me/RustMX", IconVariant::Telegram)),
    },
    Community {
        name: "Bcn Rust",
        country: "España",
        description: "Comunidad de Rust en Barcelona",
        logo: "https://rustlang-es.org/gen_assets/RustBCN.webp",
        link: Some(("https://discord.gg/DeZKCCyMcq", IconVariant::Discord)),
    },
    Community {
        name: "Rust Perú",
        country: "Perú",
        description: "¡Comunidad de Rust en Perú!",
        logo: "https://rustlang-es.org/gen_assets/rust-peru.webp",
        link: Some(("https://peru.rustlang-es.org/", IconVariant::Link)),
    },
];

#[allow(non_snake_case)]
#[island]
pub fn OtherCommunities() -> impl IntoView {
    let query = RwSignal::new("".to_string());
    let results = Signal::derive(move || {
        let query = query.read();
        COMMUNITIES
            .iter()
            .filter(|c| {
                [c.name, c.country, c.description]
                    .iter()
                    .any(|text| text.contains(query.as_str()))
            })
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
                    <For each=move || results.get() key=|community| community.name let:community>
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
