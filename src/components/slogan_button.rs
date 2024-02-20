use leptos::{create_signal, island, view, IntoView, SignalUpdate};

use rand::seq::SliceRandom;

#[island]
pub fn SloganButton() -> impl IntoView {
    let slogans = [
        "Una comunidad de gente mal intencionada y tonta.",
        "Rust the lang, not the game",
        "impl Rust for You {}",
        "También disponible en Steam",
        "Rayo Taurinizador",
        "A wild turbofish appears!",
        "Failure is not an Option<T>, it's a Result<T, E>",
        "9 de cada 10 Go dev's lo recomiendan",
        "Programmer::new()",
        "Acá le pegamos a la rústica bien recio",
        "Making the world ⚡ Blazing Fast ⚡ 🚀",
        "Rewrite it in Rust!",
        "⚡Blazingly fast⚡ 🚀 Super fast 🔥 pero ahora en español!",
        "Si te falla va ser de forma segura 😉",
        "In Rust, We Trust",
        "#![forbid(go)]",
        "Furrificando...",
        "Dumb pointer != Smart pointer",
        "(*rust)malloc(sizeof(c))",
        "Dumb Pointer != Smart Pointer",
        "std::mem::transmute::<Go, Rust>(developer)",
        "In Rust, We Trust",
        "C++ pero seguro",
        "Rust es el nuevo C",
        "Clippy >>>>>>>>>> ESLint!",
        "Null Sucks",
    ];
    let (slogan, set_slogan) =
        create_signal((*slogans.choose(&mut rand::thread_rng()).unwrap()).to_string());

    let click_handler = move |_| {
        set_slogan.update(|n| *n = (*slogans.choose(&mut rand::thread_rng()).unwrap()).to_string());
    };

    view! {
        <div
            class="flex select-none items-center justify-center lg:justify-start group"
            on:click=click_handler
        >
            <button class="bg-orange-300 border-4 border-orange-400 group-hover:border-orange-500 flex justify-center items-center rounded-full w-12 h-12 text-xl relative z-10">
                <span class="motion-safe:animate-spin">"🎲"</span>
            </button>
            <p class="font-work-sans font-light lg:text-left bg-orange-400 h-12 pr-4 pl-10 flex items-center -ml-6 flex-1 rounded-r-full group-hover:bg-orange-500 max-w-lg">
                {slogan}
            </p>
        </div>
    }
}
