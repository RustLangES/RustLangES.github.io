#![allow(dead_code)]
use leptos::{component, prelude::*, view, IntoView};

#[component]
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

    let (slogan, set_slogan) = signal(slogans[0].to_string());

    let click_handler = move |_| {
        leptos::logging::log!("Button clicked!");
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = Math)]
                fn random() -> f64;
            }
            let index = (random() * slogans.len() as f64).floor() as usize;
            leptos::logging::log!("Selected index: {}, slogan: {}", index, slogans[index]);
            set_slogan.set(slogans[index].to_string());
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use rand::seq::IndexedRandom;
            let new_slogan = (*slogans
                .choose(&mut rand::rng())
                .expect("Slogans should not be empty"))
            .to_string();
            set_slogan.set(new_slogan);
        }
    };

    view! {
        <div
            class="flex select-none items-center justify-center lg:justify-start group dark:text-white drop-shadow-[7px_7px_2px_rgba(0,0,0,.5)] hover:drop-shadow-none dark:transition-all dark:ease-in-out dark:delay-75 cursor-pointer"
            on:click=click_handler
        >
            <button
                type="button"
                class="bg-orange-200 dark:bg-black/30  border-4 border-orange-400 group-hover:border-orange-500 dark:group-hover:bg-black/60 flex justify-center items-center rounded-full w-12 h-12 text-xl relative z-10 drop-shadow-sm"
            >
                <span class="motion-safe:animate-spin">"🎲"</span>
            </button>
            <p class="font-work-sans font-light lg:text-left dark:border-double dark:hover:border-solid dark:transition-all dark:ease-in-out dark:delay-100 bg-orange-400 dark:bg-black/30 dark:border-y-4 dark:border-r-4 border-orange-400 h-12 pr-4 pl-10 flex items-center -ml-6 flex-1 rounded-r-full group-hover:bg-orange-500 dark:group-hover:bg-white/5 drop-shadow-sm  max-w-lg">
                {move || slogan.get()}
            </p>
        </div>
    }
}
