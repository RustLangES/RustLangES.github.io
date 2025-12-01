use leptos::{component, prelude::*};
use rustlanges_components::button::{Button, Variant as ButtonVariant};

pub fn DailyChallenges() -> impl IntoView {
    view! {
        <section class="bg-primary-500 dark:bg-primary-600 p-4 lg:p-20 w-full">
            <div class="container xl:min-w-[9rem] lg:px-32 mx-auto">
                <div class="bg-neutral-50 dark:bg-neutral-900 p-8 py-4 md:p-20 md:py-10 rounded-2xl drop-shadow-cards flex flex-row flex-wrap md:flex-nowrap justify-center items-stretch gap-4">
                    <div class="md:w-1/2 gap-4 lg:gap-8 flex flex-col font-body justify-between xl:p-12">
                        <div class="gap-4 flex flex-col not-md:justify-center not-md:items-center w-full">
                            <h1 class="text-heading-1 font-bold">Retos Diarios</h1>
                            <p class="text-body not-md:text-center">
                                Encuentra más ejemplos de aplicación de Rust practicando con nuestros retos diarios
                            </p>
                        </div>
                        <Button
                            variant=ButtonVariant::Primary
                            class="bg-primary-500 hidden md:block"
                            on_click=|_| {}
                            label="Practicar"
                        />
                    </div>
                    <div class="md:w-1/2 flex flex-col justify-center items-center">
                        <img src="/assets/new/images/clippy-example.png" class="rounded-xl" />

                        <Button
                            variant=ButtonVariant::Primary
                            class="bg-primary-500 md:hidden mt-4"
                            on_click=|_| {}
                            label="Practicar"
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}
