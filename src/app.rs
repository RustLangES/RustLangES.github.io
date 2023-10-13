use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    components::{Footer, Header},
    pages::{Communidad, Contributors, Index},
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <section class="w-full flex flex-col">
                <Header/>
                <main>
                    <Routes>
                        <Route path="/" view=|| view! { <Index/> }/>
                        <Route path="/colaboradores" view=|| view! { <Contributors/> }/>
                        <Route path="/Communidad" view=|| view! { <Communidad/> }/>
                    </Routes>
                    <Outlet/>
                </main>
                <Footer/>
            </section>
        </Router>
    }
}
