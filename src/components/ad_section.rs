use leptos::prelude::*;
use rustlanges_components::button::{Button, Variant as ButtonVariant};

#[component]
pub fn AdSection() -> impl IntoView {
    view! {
        <section class="w-full py-16 bg-primary-50 dark:bg-neutral-950 flex items-center justify-center">
            <div class="w-full container px-6 flex flex-col items-center justify-center m-auto gap-6">
                <h2 class="text-heading-2 text-3xl font-bold font-heading-3 text-center">
                    <span class="text-secondary-500 dark:text-primary-500">"Tu publicidad"</span>
                    " + Nuestra comunidad"
                </h2>

                <div class="gap-4 flex flex-col justify-center items-center">
                    <p class="text-heading-4 font-heading-4 font-bold max-w-xl text-center text-balance">
                        "¿Sabías que puedes publicitar tus productos o servicios en nuestros eventos y página web?"
                    </p>

                    <p class="text-heading-4 font-heading-4 text-center text-balance text-neutral-950 dark:text-neutral-50">
                        "Ponte en contacto con nosotros para conocer todos los beneficios"
                    </p>
                </div>
                <div class="flex w-full justify-center items-center">
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=|_| {}
                        class="text-heading-4 font-heading-4 font-semibold"
                        label="¡Quiero publicitar!"
                    />
                </div>
            </div>
        </section>
    }
}
