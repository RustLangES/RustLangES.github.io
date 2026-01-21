use leptos::{component, prelude::*, view, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full bg-orange-900 dark:bg-zinc-800 text-center py-6 text-white">
            "Comunidad - Rust Lang en español"
        </footer>
    }
}
