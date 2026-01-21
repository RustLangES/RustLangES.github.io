use leptos::{component, view, IntoView};
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full bg-orange-900 dark:bg-zinc-800 text-center py-6 text-white">
            "Comunidad - Rust Lang en español"
        </footer>
    }
}
