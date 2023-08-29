use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::index::Index;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route path="" view=|| view! { <Index /> }/>
            </Routes>
        </Router>
    }
}
