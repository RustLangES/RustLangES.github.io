use leptos::{component, view, IntoView};
use leptos::prelude::*;

use crate::components::{Books, HeaderAprende, Roadmap, Youtube};

#[component]
pub fn Aprende() -> impl IntoView {
    view! {
        <div class="mx-auto">
            <HeaderAprende />
            <Roadmap />
            <Books />
            <Youtube />
        </div>
    }
}
