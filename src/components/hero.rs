use crate::components::SloganButton;
use leptos::leptos_dom::logging;
use leptos::{component, prelude::*, view, IntoView};
use leptos::leptos_dom::helpers::location;

#[component]
pub fn Hero() -> impl IntoView {
    let is_in_debug_mode = cfg!(debug_assertions);

    let (uwu, set_uwu) = signal(false);

    Effect::new(move |_| {
        let location = location();
        set_uwu.set(location.search().map(|s| s.contains("uwu")).unwrap_or(false));
    });

    let image_src = move || match (is_in_debug_mode, uwu.get()) {
        (true, true) => "./assets/RustLang_uwu.png",
        (true, false) => "./assets/ferris-hero.avif",
        (false, false) => "./ferris-hero.avif",
        (false, true) => "./RustLang_uwu.png",
    };

    let class_fn = move || if !uwu.get() {
            "grid items-center py-14 lg:py-32 px-4 gap-x-20 gap-y-10 lg:grid-cols-2 w-full"
        } else {    
            "grid items-center justify-center"
        };

    view! {
        <section class=class_fn>
            <figure class="w-80 mx-auto lg:w-full">
                {move ||  
                    if !uwu.get() {
                        view! {
                            <img
                                src=image_src()
                                alt="Rust Lang en Español"
                                height="300"
                                width="500"
                                class="ml-auto"
                            />
                        }
                            .into_any()
                    } else {
                        view! {
                            <img
                                src=image_src()
                                alt="Rust Lang en Español"
                                height="700"
                                width="700"
                            />
                        }
                            .into_any()
                    }
                }
            </figure>
            <div>
                <Show when=move || !uwu.get() >
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
                </Show>
                <SloganButton uwu=uwu />
            </div>
        </section>
    }
}
