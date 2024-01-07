use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::{Footer, Header},
    pages::{Communidad, Contributors, Index},
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="es"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Rust Lang en Español"/>
        <Meta
            property="description"
            content="Somos una comunidad de Rust hispana, buscamos la promoción del lenguaje de programación Rust."
        />

        <Meta property="og:site_name" content="Rust Lang en Español"/>
        <Meta property="og:title" content="Bienvenidos a Rust Lang en Español"/>
        <Meta
            property="og:description"
            content="Somos una comunidad de Rust hispana, buscamos la promoción del lenguaje de programación Rust."
        />
        <Meta property="og:url" content="https://rustlanges.github.io"/>
        <Meta property="og:image" content="https://rustlanges.github.io/preview_concept.png"/>
        <Meta property="twitter:image" content="https://rustlanges.github.io/preview_concept.png"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:site" content="@rustlang"/>
        <Meta
            name="google-site-verification"
            content="OntIe2SKuQalaapGvxdded9tU4G2p57h0A6e0Rkoni0"
        />

        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600&family=Work+Sans:wght@300;400;500;600&display=swap"
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
