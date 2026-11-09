use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant},
    card::Card,
    icons::{Book, Project, Roadmap},
};

#[component]
pub fn OurCommunitySection() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center w-full gap-8 py-12 md:py-20 dark:bg-neutral-900">
            <div class="container px-4 flex flex-col justify-center items-center gap-6 md:gap-8">
                <h2 class="text-heading-2 md:text-heading-1 font-bold text-center">
                    Nuestra Comunidad
                </h2>
                <div class="flex flex-col gap-4 text-center text-paragraph-1 max-w-lg mb-2 md:mb-8">
                    <p>"Buscamos romper las barreras en el aprendizaje del lenguaje Rust."</p>
                    <p>
                        "Por eso "
                        <span class="font-semibold text-secondary-600 dark:text-primary-400">
                            "creamos, traducimos y difundimos"
                        </span> " material técnico " <span class="font-bold">"en español."</span>
                    </p>
                </div>

                <div class="flex flex-col md:flex-row justify-center items-stretch gap-4 w-full md:w-auto max-w-full md:flex-wrap">
                    <Card class="flex flex-col items-center justify-center gap-3 py-8 md:min-w-64 text-center">
                        <div class="bg-primary-300 border border-black rounded-xl p-2 text-black">
                            <Roadmap size=28 class="text-black" />
                        </div>
                        <h3 class="text-h4">Roadmap de Aprendizaje</h3>
                        <p class="text-paragraph-1">Descripción</p>
                    </Card>
                    <Card class="flex flex-col items-center justify-center gap-3 py-8 md:min-w-64 text-center">
                        <div class="bg-primary-300 border border-black rounded-xl p-2 text-black">
                            <Book size=28 class="text-black" />
                        </div>
                        <h3 class="text-h4">Libros y Guías traducidos</h3>
                        <p class="text-paragraph-1">Descripción</p>
                    </Card>
                    <Card class="flex flex-col items-center justify-center gap-3 py-8 md:min-w-64 text-center">
                        <div class="bg-primary-300 border border-black rounded-xl p-2 text-black">
                            <Project size=28 class="text-black" />
                        </div>
                        <h3 class="text-h4">Proyectos Open Source</h3>
                        <p class="text-paragraph-1">Descripción</p>
                    </Card>
                </div>

                <Button variant=Variant::Primary on_click=move |_| {} label="Conócenos" />
            </div>
        </section>
    }
}
