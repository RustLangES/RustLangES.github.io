use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant},
    card::Card,
    icons::{Book, Project, Roadmap},
};

#[component]
pub fn OurCommunitySection() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center w-full gap-8 py-20 dark:bg-neutral-900">
            <div class="container flex flex-col justify-center items-center gap-8">
                <h2 class="text-h2 mb-4">Nuestra Comunidad</h2>
                <p class="text-center mb-8 max-w-lg">
                    "Buscamos romper las barreras en el aprendizaje del lenguaje Rust. Por eso creamos, traducimos y difundimos material técnico en español."
                </p>

                <div class="flex flex-row justify-center items-center gap-4  max-w-full flex-wrap">
                    <Card class="flex flex-col items-center justify-center">
                        <div class="bg-orange-200 border rounded-4xl p-3 text-black">
                            <Roadmap size=40 class="text-black fill-white" />
                        </div>
                        <h3 class="text-h4">Roadmap de Aprendizaje</h3>
                        <p>Descripción</p>
                    </Card>
                    <Card class="flex flex-col items-center justify-center">
                        <div class="bg-orange-200 border rounded-4xl p-3 text-black">
                            <Book size=40 class="text-black" />
                        </div>
                        <h3 class="text-h4">Libros y Guías traducidos</h3>
                        <p>Descripción</p>
                    </Card>
                    <Card class="flex flex-col items-center justify-center">
                        <div class="bg-orange-200 border rounded-4xl p-3 ">
                            <Project size=40 class="text-black" />
                        </div>
                        <h3 class="text-h4">Proyectos Open Source</h3>
                        <p>Descripción</p>
                    </Card>
                </div>

                <Button variant=Variant::Primary on_click=move |_| {} label="Conócenos" />
            </div>
        </section>
    }
}
