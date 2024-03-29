use crate::components::SloganButton;
use leptos::{component, view, IntoView};

#[component]
pub fn Hero() -> impl IntoView {
    let image_src = if cfg!(debug_assertions) {
        "./assets/ferris-hero.avif"
    } else {
        "/ferris-hero.avif"
    };

    view! {
        <section class="grid items-center py-14 lg:py-32 px-4 gap-x-20 gap-y-10 lg:grid-cols-2 w-full">
            <figure class="w-80 mx-auto lg:w-full">
                <img
                    src=image_src
                    alt="Rust Lang en Español"
                    height="300"
                    width="500"
                    class="ml-auto"
                />
            </figure>
            <div>
                <h1 class="flex flex-col mb-4 gap-y-2">
                    <span class="font-work-sans text-4xl font-light text-center lg:text-left">
                        "Bienvenidos a"
                    </span>
                    <span class="font-alfa-slab text-orange-500 dark:text-orange_(pantone)-500 text-6xl sm:text-7xl lg:text-8xl text-center lg:text-left">
                        "Rust Lang"
                    </span>
                    <span class="font-work-sans text-5xl font-semibold text-center lg:text-left">
                        "En Español"
                    </span>
                </h1>
                <SloganButton/>
            </div>
        </section>
    }
}
