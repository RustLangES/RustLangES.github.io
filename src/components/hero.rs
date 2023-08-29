use leptos::*;

use crate::components::header::Header;
use rand::seq::SliceRandom;

#[component]
pub fn Hero() -> impl IntoView {
    let slogans = [
        "Una comunidad de gente mal intencionada y tonta.",
        "9 de cada 10 Go dev's lo recomiendan",
        "Acá le pegamos a la rústica bien recio",
        "⚡ Blazingly fast ⚡ 🚀🚀🚀 Super fast 🔥🔥🔥🔥 pero ahora en español!!",
        "Si te falla va ser de forma segura 😉",
        "Furrificando...",
    ];
    let (slogan, set_slogan) =
        create_signal(slogans.choose(&mut rand::thread_rng()).unwrap().to_string());

    let click_handler = move |_| {
        set_slogan.update(|n| *n = slogans.choose(&mut rand::thread_rng()).unwrap().to_string());
    };

    view! {
        <section class="w-full flex flex-col">
            <Header/>
            <div class="flex items-center justify-center py-14 lg:py-32 px-4">
                <div class="grid items-center gap-x-20 gap-y-10 lg:grid-cols-2">
                    <figure class="w-80 mx-auto lg:w-full">
                        <img src="./rhq3ezvso9611-min.png" width="500" class="mx-auto"/>
                    </figure>
                    <div class="">
                        <h1 class="flex flex-col mb-4 gap-y-2">
                            <span class="font-work-sans text-4xl font-light text-center lg:text-left">
                                "Bienvenidos a"
                            </span>
                            <span class="font-alfa-slab text-orange-500 text-6xl sm:text-7xl lg:text-8xl text-center lg:text-left">
                                "Rust Lang"
                            </span>
                            <span class="font-work-sans text-5xl font-semibold text-center lg:text-left">
                                "En Español"
                            </span>
                        </h1>
                        // TODO: Alguien que sepa de diseño que me ayude a hacer esto mas bonito
                        <button
                            class=" border-black p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition flex justify-center items-center rounded-[100%]  w-6 h-6 mx-auto"
                            on:click=click_handler
                        >
                            <span class="motion-safe:animate-spin">"🎲"</span>
                        </button>
                        <p class="font-work-sans font-light text-center lg:text-left max-w-[90%]">
                            {slogan}
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
