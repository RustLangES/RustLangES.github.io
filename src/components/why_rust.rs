use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant as ButtonVariant},
    card::Card,
    icons::Youtube,
};

#[component]
pub fn WhyRust() -> impl IntoView {
    view! {
        <section class="bg-white dark:bg-neutral-900 py-16 px-4 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col md:flex-row gap-12 flex-wrap md:flex-nowrap justify-center items-center">
                <div class="w-full md:w-1/2 flex flex-col gap-6">
                    <h2 class="text-5xl font-heading-1 font-bold">Nosotros</h2>
                    <p class="text-lg font-semibold leading-relaxed">
                        "Somos una "
                        <span class="text-primary-500 dark:text-primary-400">"comunidad de Rust hispana"</span>
                        " que busca la promoción de este lenguaje."
                    </p>
                    <div>
                        <Button
                            variant=ButtonVariant::Secondary
                            label="Visita nuestro canal"
                            icon=view! { <Youtube /> }.into_any()
                            on_click=|_| {}
                        />
                    </div>
                </div>

                <div class="w-full md:w-1/2">
                    <Card class="p-0 overflow-clip">
                        <h3 class="text-heading-3 font-heading-3 font-bold text-akira bg-neutral-950 text-white text-center p-4 dark:text-primary-400">
                            "¿Por qué Rust?"
                        </h3>
                        <p class="text-body font-body p-8 leading-relaxed">
                            "Rust es un lenguaje de programación de sistemas que se enfoca en la "
                            <span class="font-bold">"seguridad"</span>
                            ", la "
                            <span class="font-bold">"velocidad"</span>
                            " y la "
                            <span class="font-bold">"concurrencia"</span>
                            ". Su sintaxis es similar a la de C++, pero introduce conceptos nuevos que lo hacen "
                            <span class="font-bold">"más seguro y más fácil de usar"</span>
                            "."
                        </p>
                    </Card>
                </div>
            </div>
        </section>
    }
}
