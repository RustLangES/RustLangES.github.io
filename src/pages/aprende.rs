use leptos::prelude::*;
use rustlanges_components::button::{Button, Variant as ButtonVariant};

use crate::components::resources::Resources;

#[component]
pub fn Aprende() -> impl IntoView {
    view! {
        <div class="w-full h-[65dvh] rustlang-es-background-secondary dark:bg-[#3E1C96CC] text-akira flex items-center justify-center">
            <div class="w-full container  flex  flex-col-reverse md:flex-row items-center justify-center m-auto">
                <div class="flex flex-col justify-center gap-8">
                    <div class="flex flex-col gap-2 justify-center">
                        <p class="uppercase text-h1">Aprende Rust</p>
                        <p class="uppercase text-h1 text-primary-500">En Español</p>
                        <p>"Roadmap de aprendizaje, libros, guías y más."</p>
                        <p>"Explora todos los recursos y ¡empieza a tu viaje! 🚀"</p>
                    </div>
                    <div class="flex gap-4 flex-wrap max-w-full">
                        <Button
                            variant=ButtonVariant::Primary
                            class="bg-light"
                            on_click=|_| {}
                            label="Ir al Roadmap"
                        />
                    </div>
                </div>
                <div>
                    <img
                        src="/assets/new/logos/logo-minified.svg"
                        alt=""
                        class="max-w-3xs lg:max-w-md"
                    />
                </div>
            </div>
        </div>
        <Resources />
    }
}
