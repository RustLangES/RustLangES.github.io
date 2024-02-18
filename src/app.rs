use leptos::{component, tracing, view, IntoView};
use leptos_meta::{provide_meta_context, Body, Html, Meta, Stylesheet, Title};
use leptos_router::{Router, Routes, StaticParamsMap, StaticRoute};

use crate::{
    components::{Footer, Header},
    pages::{Aprende, Communidad, Contributors, Index},
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let assets_folder = if cfg!(debug_assertions) {
        "./assets"
    } else {
        "."
    };

    view! {
        <Html lang="es"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Stylesheet id="fonts" href=format!("{}/fonts.css", assets_folder)/>
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Rust Lang en Español"/>
        <Meta
            name="description"
            content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
        />

        <Meta name="og:site_name" content="Rust Lang en Español"/>
        <Meta name="og:title" content="Bienvenidos a Rust Lang en Español"/>
        <Meta
            name="og:description"
            content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
        />
        <Meta name="og:url" content="https://rustlanges.github.io"/>
        <Meta name="og:image" content="https://rustlanges.github.io/preview_concept.png"/>
        <Meta name="twitter:image" content="https://rustlanges.github.io/preview_concept.png"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:site" content="@rustlang"/>
        <Meta
            name="google-site-verification"
            content="OntIe2SKuQalaapGvxdded9tU4G2p57h0A6e0Rkoni0"
        />

        <Body class="bg-orange-200"/>
        <Router>
            <Header/>
            <main>
                <Routes>
                    <StaticRoute
                        path="/"
                        view=Index
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/comunidad"
                        view=Communidad
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/colaboradores"
                        view=Contributors
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/aprende"
                        view=Aprende
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="404"
                        view=Index
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
