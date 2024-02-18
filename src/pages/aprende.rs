use leptos::{IntoView, component, tracing, view};

use crate::components::{Books, HeaderAprende, Roadmap, Youtube};

#[component]
pub fn Aprende() -> impl IntoView {
    view! {
        <div class="mx-auto">
            <HeaderAprende/>
            <Roadmap/>
            <Books/>
            <Youtube/>
        </div>
    }
}