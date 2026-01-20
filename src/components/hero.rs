use crate::components::SloganButton;
use leptos::{island, view, IntoView, SignalGet};
use leptos_router::use_query_map;

#[island]
pub fn Hero() -> impl IntoView {
    let query_params = use_query_map();

    let is_in_debug_mode = cfg!(debug_assertions);
    let uwu = query_params.get().get("uwu").is_some();

    let image_src = match (is_in_debug_mode, uwu) {
        (true, true) => "./assets/RustLang_uwu.png",
        (true, false) => "./assets/ferris-hero.avif",
        (false, false) => "./ferris-hero.avif",
        (false, true) => "./RustLang_uwu.png",
    };

    view! {
        <section
            class=(
                "grid items-center py-14 lg:py-32 px-4 gap-x-20 gap-y-10 lg:grid-cols-2 w-full",
                move || !uwu,
            )
            class="grid items-center justify-center"
        >
            <figure class="w-80 mx-auto lg:w-full">
                {if !uwu {
                    view! {
                        <img
                            src=image_src
                            alt="Rust Lang en Español"
                            height="300"
                            width="500"
                            class="ml-auto"
                        />
                    }
                } else {
                    view! {
                        <img src=image_src alt="Rust Lang en Español" height="700" width="700" />
                    }
                }}
            </figure>
            <div>
                {if !uwu {
                    view! {
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
                    }
                } else {
                    view! { <h1 class="hidden">"UwU"</h1> }
                }} <SloganButton uwu=uwu />
            </div>
        </section>
    }
}
