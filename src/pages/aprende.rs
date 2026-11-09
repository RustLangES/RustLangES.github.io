use leptos::prelude::*;
use rustlanges_components::button::{Button, Variant as ButtonVariant};

use crate::components::{
    challenges::DailyChallenges, channels::Channels, footer::Footer, resources::Resources,
};

#[component]
pub fn Aprende() -> impl IntoView {
    view! {
        <div class="w-full md:min-h-[65dvh] py-12 md:py-0 rustlang-es-background-secondary dark:bg-[#3E1C96CC] flex items-center justify-center">
            <div class="w-full container desktop:max-w-desktop px-4 flex flex-col-reverse md:flex-row items-center justify-center gap-8 m-auto">
                <div class="flex flex-col justify-center items-center md:items-start gap-8 text-center md:text-left">
                    <div class="flex flex-col gap-4">
                        <p class="uppercase text-akira heading text-[28px] tablet:text-[32px] desktop:text-[48px]">
                            "Aprende Rust" <br />
                            <span class="text-primary-500">"En Español"</span>
                        </p>
                        <p class="text-subtitle-1 max-w-md">
                            "Roadmap de aprendizaje, libros, guías y más. Explora todos los recursos y ¡empieza tu viaje! 🚀"
                        </p>
                    </div>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=|_| {}
                        label="Ir al Roadmap"
                    />
                </div>
                <img
                    src="/assets/new/logos/logo-minified.svg"
                    alt=""
                    class="w-40 tablet:w-56 md:w-auto md:max-w-3xs lg:max-w-md"
                />
            </div>
        </div>
        <Resources />
        <DailyChallenges />
        <Channels />
        <Footer />
    }
}
