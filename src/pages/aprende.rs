use leptos::prelude::*;
use rustlanges_components::button::{Button, Variant as ButtonVariant};

use crate::components::{
    challenges::DailyChallenges,
    channels::Channels,
    footer::Footer,
    resources::Resources,
};

#[component]
pub fn Aprende() -> impl IntoView {
    view! {
        // Hero - lavender/purple background
        <div class="w-full min-h-[65dvh] rustlang-es-background-secondary dark:bg-[#3E1C96CC] text-akira flex items-center justify-center py-16">
            <div class="w-full container flex flex-col md:flex-row items-center justify-center m-auto gap-8 lg:gap-16 px-6">
                <div class="flex flex-col justify-center gap-8 mx-auto lg:mx-0 text-center lg:text-left">
                    <div class="flex flex-col gap-2">
                        <h1 class="uppercase leading-tight">"Aprende Rust"</h1>
                        <h1 class="uppercase leading-tight text-primary-500 dark:text-orange-300">
                            "En Español"
                        </h1>
                        <p class="text-base font-normal font-body mt-2 leading-relaxed max-w-md">
                            "Roadmap de aprendizaje, libros, guías y más."
                        </p>
                        <p class="text-base font-normal font-body">
                            "Explora todos los recursos y ¡empieza a tu viaje! 🚀"
                        </p>
                    </div>
                    <div class="flex gap-4 flex-wrap justify-center lg:justify-start">
                        <Button
                            variant=ButtonVariant::Secondary
                            on_click=|_| {}
                            label="Ir al Roadmap"
                        />
                    </div>
                </div>
                <div class="flex-shrink-0">
                    <img
                        src="/assets/new/logos/logo-minified.svg"
                        alt="Logo RustLangES"
                        class="w-48 lg:w-72 xl:w-96"
                    />
                </div>
            </div>
        </div>

        <Resources />
        <DailyChallenges />
        <Channels />
        <Footer />
    }
}
