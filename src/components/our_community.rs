use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant},
    card::Card,
    icons::{Book, Project, Roadmap},
};

#[component]
pub fn OurCommunitySection() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center w-full gap-8 py-20 bg-white dark:bg-neutral-900">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col justify-center items-center gap-8">
                <h2 class="text-h2">"Nuestra Comunidad"</h2>
                <p class="text-center max-w-lg leading-relaxed">
                    "Buscamos romper las barreras en el aprendizaje del lenguaje Rust."
                    <br />
                    "Por eso "
                    <span class="text-secondary-600 dark:text-secondary-400 font-medium">
                        "creamos, traducimos y difundimos"
                    </span>
                    " material técnico "
                    <span class="font-bold">"en español"</span>
                    "."
                </p>

                <div class="flex flex-row justify-center items-stretch gap-6 max-w-full flex-wrap md:flex-nowrap">
                    <Card class="flex flex-col items-center justify-start gap-4 p-6 text-center max-w-xs">
                        <div class="bg-orange-200 dark:bg-orange-900/50 border border-orange-300 rounded-2xl p-3">
                            <Roadmap size=40 class="text-black dark:text-orange-300" />
                        </div>
                        <h3 class="text-h4 font-bold">"Roadmap de Aprendizaje"</h3>
                        <p class="text-sm text-neutral-600 dark:text-neutral-400">"Descripción"</p>
                    </Card>
                    <Card class="flex flex-col items-center justify-start gap-4 p-6 text-center max-w-xs">
                        <div class="bg-orange-200 dark:bg-orange-900/50 border border-orange-300 rounded-2xl p-3">
                            <Book size=40 class="text-black dark:text-orange-300" />
                        </div>
                        <h3 class="text-h4 font-bold">"Libros y Guías traducidos"</h3>
                        <p class="text-sm text-neutral-600 dark:text-neutral-400">"Descripción"</p>
                    </Card>
                    <Card class="flex flex-col items-center justify-start gap-4 p-6 text-center max-w-xs">
                        <div class="bg-orange-200 dark:bg-orange-900/50 border border-orange-300 rounded-2xl p-3">
                            <Project size=40 class="text-black dark:text-orange-300" />
                        </div>
                        <h3 class="text-h4 font-bold">"Proyectos Open Source"</h3>
                        <p class="text-sm text-neutral-600 dark:text-neutral-400">"Descripción"</p>
                    </Card>
                </div>

                <Button variant=Variant::Primary on_click=move |_| {} label="Conócenos" />
            </div>
        </section>
    }
}
