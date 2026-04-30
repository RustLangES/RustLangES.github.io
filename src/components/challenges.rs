use leptos::prelude::*;
use rustlanges_components::button::{Button, Variant as ButtonVariant};

#[component]
pub fn DailyChallenges() -> impl IntoView {
    view! {
        <section class="bg-primary-500 dark:bg-primary-600 py-16 px-4 w-full">
            <div class="container max-w-7xl mx-auto">
                <div class="bg-neutral-50 dark:bg-neutral-900 rounded-2xl drop-shadow-cards flex flex-col md:flex-row justify-center items-stretch overflow-hidden">
                    <div class="md:w-1/2 p-8 md:p-12 flex flex-col gap-6 justify-between">
                        <div class="flex flex-col gap-4">
                            <h2 class="text-heading-1 font-bold">"Retos Diarios"</h2>
                            <p class="text-body leading-relaxed">
                                "Encuentra más ejemplos de aplicación de Rust practicando con nuestros retos diarios"
                            </p>
                        </div>
                        <div class="hidden md:block">
                            <Button
                                variant=ButtonVariant::Primary
                                on_click=|_| {}
                                label="Practicar"
                            />
                        </div>
                    </div>
                    <div class="md:w-1/2 flex flex-col justify-center items-center p-4 md:p-8 bg-neutral-100 dark:bg-neutral-800">
                        <img
                            src="/assets/new/images/clippy-example.png"
                            alt="Ejemplo de Clippy en terminal"
                            class="rounded-xl w-full max-w-lg"
                        />
                        <div class="mt-4 md:hidden">
                            <Button
                                variant=ButtonVariant::Primary
                                on_click=|_| {}
                                label="Practicar"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
