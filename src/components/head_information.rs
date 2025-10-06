use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_meta::{Link, Meta, Stylesheet, Title};
use leptos_router::hooks::use_location;

#[cfg(debug_assertions)]
const ASSETS_FOLDER: &str = "./assets";

#[cfg(not(debug_assertions))]
const ASSTES_FOLDERS: &str = ".";

#[component]
pub fn HeadInformation() -> impl IntoView {
    let location = use_location().pathname.read();
    let path = location.as_str();

    let preview = if path == "/aprende" {
        "aprende_preview.webp"
    } else {
        "rustlanges_preview.webp"
    };

    view! {
        <>
            <Meta charset="utf-8"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1"/>
            <Stylesheet id="fonts" href=format!("{ASSETS_FOLDER}/fonts.css")/>
            <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
            <Title text="Rust Lang en Español"/>
            <Meta
                name="description"
                content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
            />

            <Meta property="og:site_name" content="Rust Lang en Español"/>
            <Meta property="og:title" content="Bienvenidos a Rust Lang en Español"/>
            <Meta
                property="og:description"
                content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
            />
            <Meta property="og:url" content="https://rustlang-es.org"/>

            <Meta property="twitter:card" content="summary_large_image"/>
            <Meta property="twitter:site" content="@rustlang"/>
            <Meta
                property="google-site-verification"
                content="OntIe2SKuQalaapGvxdded9tU4G2p57h0A6e0Rkoni0"
            />

            <Link rel="canonical" href=format!("https://rustlang-es.org{path}")/>
            <Meta property="og:image" content=format!("https://rustlang-es.org/{preview}")/>
            <Meta property="twitter:image" content=format!("https://rustlang-es.org/{preview}")/>
        </>
    }
}
