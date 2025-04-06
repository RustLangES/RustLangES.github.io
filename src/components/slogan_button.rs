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
        "(*rust)malloc(sizeof(c))",
        "Dumb Pointer != Smart Pointer",
        "std::mem::transmute::<Go, Rust>(developer)",
        "C++ pero seguro",
        "Rust es el nuevo C",
        "Clippy >>>>>>>>>> ESLint!",
        "Null Sucks",
        "rust.cmp(&cpp) == Ordering::Greater",
        "El codigo entra por los dedos, a programar para aprender",
    ];
    let (slogan, set_slogan) =
        create_signal((*slogans.choose(&mut rand::thread_rng()).unwrap()).to_string());

    let click_handler = move |_| {
        set_slogan.update(|n| *n = (*slogans.choose(&mut rand::thread_rng()).unwrap()).to_string());
    };

    view! {
        <div
            class="flex select-none items-center justify-center lg:justify-start group dark:text-white drop-shadow-[7px_7px_2px_rgba(0,0,0,.5)] hover:drop-shadow-none dark:transition-all dark:ease-in-out  dark:delay-75 "
            on:click=click_handler
        >
            <button class="bg-orange-200 dark:bg-black/30  border-4 border-orange-400 group-hover:border-orange-500 dark:group-hover:bg-black/60 flex justify-center items-center rounded-full w-12 h-12 text-xl relative z-10 drop-shadow-sm">
                <span class="motion-safe:animate-spin">"🎲"</span>
            </button>
            <p class="font-work-sans font-light lg:text-left dark:border-double dark:hover:border-solid dark:transition-all dark:ease-in-out dark:delay-100 bg-orange-400 dark:bg-black/30 dark:border-y-4 dark:border-r-4 border-orange-400 h-12 pr-4 pl-10 flex items-center -ml-6 flex-1 rounded-r-full group-hover:bg-orange-500 dark:group-hover:bg-white/5 drop-shadow-sm  max-w-lg">
                {slogan}
            </p>
        </div>
    }
}
