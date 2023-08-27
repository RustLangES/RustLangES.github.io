use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::index::Index;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/LogoSegunMichael.ico"/>
        <Router>
            <Routes>
                <Route path="" view=|| view! { <Index /> }/>
            </Routes>
        </Router>
    }
}

