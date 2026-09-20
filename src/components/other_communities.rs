use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Variant as ButtonVariant, button_class},
    card::Card,
    chip::{Chip, Variant as ChipVariant},
    icons::IconVariant,
    input::{Filter, InputSearch},
};

type ResourceAlias = Vec<Filter>;

// TODO(@Lemin-n): quitar el pub, please
struct Community {
    pub logo: &'static str,
    pub name: &'static str,
    pub country: &'static str,
    pub description: &'static str,
    pub link: Option<(&'static str, IconVariant)>,
}

const COMMUNITIES: [Community; 7] = [
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
pub fn OtherCommunities() -> impl IntoView {
    let communities = COMMUNITIES.map(|community| {
        let button_link = community.link.map(|(link, link_icon)| {
            view! {
                <a
                    class=button_class(
                        ButtonVariant::Icon,
                        Some("-top-3 -right-2 absolute".to_string()),
                    )
                    href=link
                >
                    {link_icon.to_view()}
                </a>
            }
        });

        view! {
            <div class="min-w-87.5 max-w-md h-full mt-5">
                <Card class="min-w-87.5 max-w-md h-full p-4">
                    <div class="flex flex-col gap-4 justify-between h-full">
                        <div class="flex flex-col items-center gap-4  h-full justify-stretch p-2">
                            <div class="relative">
                                {button_link} <Avatar url=community.logo size=100 />
                            </div>
                            <h2 class="text-h3 mt-2">{community.name}</h2>
                            <Chip label=community.country variant=ChipVariant::Location />
                            <p class="text-paragraph-3 mb-4 text-center text-balance mx-auto">
                                {community.description}
                            </p>
                        </div>
                    </div>
                </Card>
            </div>
        }
    });

    view! {
        <section class="bg-orange-100 dark:bg-neutral-950 p-4 lg:p-20 w-full">
            <div class="flex flex-col justify-center items-center w-full container xl:max-w-7xl lg:px-36 lg:pb-5 mx-auto">
                <h2 class="text-h2 text-center mb-4">"Otras Comunidades Rust"</h2>
                <div>
                    <p class="text-center max-w-lg">"Encuentra la comunidad más cercana a ti"</p>
                </div>
                <div class="flex flex-col justify-center mb-8 container mt-10 mx-auto max-w-fit">
                    // TODO: Implement functionality referenced in the issue: https://github.com/RustLangES/RustLangES.github.io/issues/113
                    <InputSearch on_change_filter=move |_resource: ResourceAlias| () />
                </div>
            </div>
            <div class="flex flex-row justify-center items-center gap-4 max-w-full m-auto">
                <div class="items-center gap-8 overflow-x-auto p-4 grid md:grid-cols-2 xl:grid-cols-3">
                    {communities}
                </div>
            </div>
        </section>
    }
}
