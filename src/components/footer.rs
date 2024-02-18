use leptos::{IntoView, component, tracing, view};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full bg-orange-900 text-center py-6 text-white">
            "Comunidad - Rust Lang en español"
        </footer>
    }
}