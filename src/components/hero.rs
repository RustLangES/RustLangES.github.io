use leptos::*;

use rand::seq::SliceRandom;

#[component]
pub fn Hero() -> impl IntoView {
    let slogans = [
        "Una comunidad de gente mal intencionada y tonta.",
        "9 de cada 10 Go dev's lo recomiendan",
        "Acá le pegamos a la rústica bien recio",
        "⚡Blazingly fast⚡ 🚀 Super fast 🔥 pero ahora en español!",
        "Si te falla va ser de forma segura 😉",
        "Furrificando...",
        "(*rust)malloc(sizeof(c))",
        "Dumb Pointer != Smart Pointer",
        "std::mem::transmute::<Go, Rust>(developer)",
        "In Rust, We Trust",
        "Rust es el nuevo C",
        "Clippy >>>>>>>>>> ESLint!",
        "Null Sucks",
    ];
    let (slogan, set_slogan) =
        create_signal(slogans.choose(&mut rand::thread_rng()).unwrap().to_string());

    let click_handler = move |_| {
        set_slogan.update(|n| *n = slogans.choose(&mut rand::thread_rng()).unwrap().to_string());
    };

    view! {
        <div class="flex items-center justify-center py-14 lg:py-32 px-4">
            <div class="grid items-center gap-x-20 gap-y-10 lg:grid-cols-2 w-full">
                <figure class="w-80 mx-auto lg:w-full">
                    <img
                        src="./rhq3ezvso9611-min.png"
                        alt="Rust Lang en Español"
                        width="500"
                        class="ml-auto"
                    />
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
                    <div class="flex items-center justify-center lg:justify-start group"
                            on:click=click_handler
                    >
                        <button
                            class="bg-orange-300 border-4 border-orange-400 group-hover:border-orange-500 flex justify-center items-center rounded-full w-12 h-12 text-xl relative z-10"
                        >
                            <span class="motion-safe:animate-spin">"🎲"</span>
                        </button>
                        <p class="font-work-sans font-light lg:text-left bg-orange-400 h-12 pr-4 pl-10 flex items-center -ml-6 flex-1 rounded-r-full group-hover:bg-orange-500 max-w-lg">
                            {slogan}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
