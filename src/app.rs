use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::index::Index;
use crate::components::layout::header::Header;



#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Header />
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Index/> }/>
            </Routes>
        </Router>
    }
}

