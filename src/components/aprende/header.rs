use leptos::{component, view, IntoView};

#[component]
pub fn HeaderAprende() -> impl IntoView {
    view! {
        <header class="border-b border-b-black/20 dark:border-b-transparent bg-orange-100 dark:bg-transparent py-20 flex ">
            <div class="container mx-auto">
                <h1 class="font-alfa-slab text-6xl sm:text-7xl lg:text-8xl text-center lg:text-left">
                    "Aprende Rust"
                </h1>
            </div>
        </header>
    }
}
