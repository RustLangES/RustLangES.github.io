use leptos::*;

use crate::components::cards::community_card::CommunityCard;

struct CommunityItem {
    name: Vec<&'static str>,
    description: &'static str,
    link: &'static str,
    icon: &'static str,
    brand_src: &'static str,
    brand_alt: &'static str,
}

#[component]
pub fn OtherCommunities() -> impl IntoView {
    let items: Vec<CommunityItem> = vec![
        CommunityItem {
            name: vec!["Rust ", "Barcelona"],
            description: "Comunidad de Rust en Barcelona",
            link: "https://discord.gg/DeZKCCyMcq",
            icon: "discord",
            brand_src: "./RustBCN.webp",
            brand_alt: "Logo de Rust Barcelona",
        },
        CommunityItem {
            name: vec!["Whizzles"],
            description: "Comunidad de Open Source con fuertes raices en Rust.",
            link: "https://discord.gg/b9EbVnSkuw",
            icon: "discord",
            brand_src: "./Whizzles.webp",
            brand_alt: "Logo de Whizzles",
        },
        CommunityItem {
            name: vec!["Rust", "MX"],
            description: "Comunidad de Rust en Mexico",
            link: "https://t.me/RustMX",
            icon: "telegram",
            brand_src: "./RustMX-min.png",
            brand_alt: "Logo de RustMX",
        },
        CommunityItem {
            name: vec!["Aprende ", "Rust"],
            description: "Comunidad de Rust en Mexico",
            link: "https://t.me/aprenderrust",
            icon: "telegram",
            brand_src: "./aprender-rust-min.jpg",
            brand_alt: "Logo de Aprende Rust",
        },
        CommunityItem {
            name: vec!["Rust ", "Español"],
            description: "Anteriormente Rust Argentina, hoy una comunidad internacional.",
            link: "https://t.me/rust_lang_es",
            icon: "telegram",
            brand_src: "./RustGeneral.jpg",
            brand_alt: "Logo de Rust Español",
        },
        CommunityItem {
            name: vec!["Salamandra ", "Devs"],
            description: "Comunidad de Programación en general.",
            link: "https://t.me/salamandradevs",
            icon: "telegram",
            brand_src: "./Salamandra.png",
            brand_alt: "Logo de Salamandra Devs",
        },
    ];

    view! {
        <section class="bg-orange-100 py-20">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl text-left mb-6">
                    <span class="font-work-sans font-light">"Otras "</span>
                    <span class="font-alfa-slab text-orange-500">"Comunidades"</span>
                    <span class="font-work-sans font-light">" recomendadas "</span>
                </h2>
                <div class="w-full grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-x-8 gap-y-8">
                    {items
                        .into_iter()
                        .map(|item| {
                            view! {
                                <CommunityCard
                                    name=item.name.clone()
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
