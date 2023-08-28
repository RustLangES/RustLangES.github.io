use leptos::*;

use crate::components::cards::project_card::ProjectCard;

struct ProjectItem {
    name: Vec<&'static str>,
    description: &'static str,
    link: &'static str,
    brand_src: &'static str,
    button_link: &'static str,
    button_text: &'static str,
    brand_as_letter: bool,
    button_bg_color: &'static str,
}

#[component]
pub fn CommunityProjects() -> impl IntoView {
    let items: Vec<ProjectItem> = vec![
        ProjectItem {
            name: vec!["Rust ", "Lang en Español"],
            description: "Pagina de la comunidad de Rust en Español",
            link: "https://github.com/RustLangES/rust-book-es",
            brand_src: "https://www.rust-lang.org/static/images/rust-logo-blk.svg",
            button_link: "https://github.com/RustLangES",
            button_text: "",
            brand_as_letter: false,
            button_bg_color: "white",
        },
        ProjectItem {
            name: vec!["The ", "Rust", " Programming Language [Spanish Ed.]"],
            description: "Traducción del libro oficial de Rust al español",
            link: "https://github.com/RustLangES/rust-book-es",
            brand_src: "./RustLogo.png",
            button_link: "https://github.com/RustLangES",
            button_text: "Rust Lang en Español",
            brand_as_letter: false,
            button_bg_color: "white",
        },
        ProjectItem {
            name: vec!["Hereditary"],
            description: "Una libreria para emular herencia en Rust",
            link: "https://github.com/datapture/hereditary",
            brand_src: "H",
            button_link: "https://github.com/superoptimo",
            button_text: "Francisco Leon",
            brand_as_letter: true,
            button_bg_color: "black",
        },
        ProjectItem {
            name: vec!["gabble"],
            description: "Un sistema de chats en tiempo real",
            link: "https://github.com/whizzes/gabble",
            brand_src: "🎎",
            button_link: "https://github.com/EstebanBorai",
            button_text: "Esteban Borai",
            brand_as_letter: true,
            button_bg_color: "white",
        },
        ProjectItem {
            name: vec!["Graphul"],
            description: "Un framework web basado en Axum",
            link: "https://graphul-rs.github.io",
            brand_src: "https://graphul-rs.github.io/img/logo.svg",
            button_link: "https://github.com/SamuelBonilla",
            button_text: "Samuel Bonilla",
            brand_as_letter: false,
            button_bg_color: "black",
        },
        ProjectItem {
            name: vec!["http-server"],
            description: "Un command line HTTP server simple y configurable",
            link: "https://github.com/http-server-rs/http-server",
            brand_src: "https://github.com/http-server-rs/http-server/raw/main/assets/logo.svg",
            button_link: "https://github.com/EstebanBorai",
            button_text: "Esteban Borai",
            brand_as_letter: false,
            button_bg_color: "white",
        },
        ProjectItem {
            name: vec!["Freya"],
            description: "Una libreria para hacer GUI nativas utilizando Dioxus y Skia",
            link: "https://github.com/marc2332/freya",
            brand_src: "https://github.com/marc2332/freya/raw/main/logo.svg",
            button_link: "https://github.com/marc2332",
            button_text: "Marc Espín",
            brand_as_letter: false,
            button_bg_color: "white",
        },
    ];

    view! {
        <section class="bg-orange-200 py-20">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl text-left mb-6">
                    <span class="font-work-sans font-light">"Proyectos de la "</span>
                    <span class="font-alfa-slab text-orange-500">"Comunidad"</span>
                </h2>
                <div class="w-full grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-x-8 gap-y-4">
                    {items.into_iter().map(|item| {
                        view! {
                            <ProjectCard
                                name=item.name
                                description=item.description
                                link=item.link
                                brand_src=item.brand_src
                                button_link=item.button_link
                                button_text=item.button_text
                                brand_as_letter=item.brand_as_letter
                                button_bg_color=item.button_bg_color
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
