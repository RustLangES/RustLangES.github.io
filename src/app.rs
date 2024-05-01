use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Body};
use leptos_router::{Router, Routes, StaticParamsMap, StaticRoute};

use crate::{
    components::{Footer, HeadInformation, Header},
    pages::{Aprende, Communities, Contributors, Index, Projects},
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let bg_in_dark_mode = if cfg!(debug_assertions) {
        "dark:bg-kaku-dev"
    } else {
        "dark:bg-kaku"
    };

    view! {
        <Router>
            <HeadInformation/>
            <Body class=format!(
                "bg-orange-200 dark:bg-[#131313]/90 bg-center bg-fixed {} dark:bri dark:bg-cover dark:bg-blend-darken dark:backdrop-blur-xl overflow-x-hidden dark:text-[#e2cea9]",
                bg_in_dark_mode,
            )/>
            <Header/>
            <main>
                <Routes>
                    <StaticRoute
                        path="/"
                        view=Index
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/comunidades"
                        view=Communities
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/colaboradores"
                        view=Contributors
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/proyectos"
                        view=Projects
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
