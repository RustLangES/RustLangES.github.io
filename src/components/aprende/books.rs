use leptos::{IntoView, component, view};

use crate::components::ButtonLink;

#[component]
pub fn Books(
) -> impl IntoView {
    view! {
        <section class=" bg-orange-300/30 py-20">
            <div class="container mx-auto mb-5">
                <h1 class="font-alfa-slab text-3xl sm:text-4xl lg:text-5xl text-center mb-5">
                    "¡Nuestros Libros!"
                </h1>
                <p class="max-w-screen-xl mx-auto text-center text-balance text-lg">
                    "Hemos dedicado tiempo y esfuerzo a adaptar libros al español, estamos orgullosos de compartir estos recursos, esperando que contribuyan al acceso y comprensión de valiosos conocimientos."
                </p>
            </div>
            <div class="flex flex-row justify-center items-center  container mx-auto mb-28">
                <section class="w-1/2 px-10">
                    <div class="relative group flex flex-col gap-y-6 border border-black p-2 sm:p-6 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] justify-between group transition-all transform">
                        <span class="absolute top-0 end-0 inline-flex items-center size-3.5 rounded-full border-2 border-white text-xs font-medium transition-all transform -translate-y-1/2 translate-x-1/2 bg-teal-500 dark:border-slate-900 badge-container">
                            <span class="sr-only text-black badge-content transition-all transform ">
                                "Recomendado"
                            </span>
                        </span>
                        <h1 class="font-alfa-slab text-xl sm:text-2xl lg:text-3xl text-center mb-5">
                            "El Lenguaje de Programación Rust"
                        </h1>
                        <p class="container mx-auto">
                            "Cariñosamente conocido como “el libro”, El Lenguaje de Programación Rust te dará una visión del lenguaje desde los principios básicos. Construirás unos cuantos proyectos por el camino y, al final, tendrás una comprensión sólida del lenguaje."
                        </p>
                        <div class="mx-auto">
                            <ButtonLink href="https://rustlanges.github.io/rust-book-es" size="big">
                                "Ir a “El Libro”"
                            </ButtonLink>
                        </div>
                    </div>
                </section>

                <section class="w-1/2 px-10">
                    <div class="group flex flex-col gap-y-6 border border-black p-2 sm:p-6 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between">
                        <span class="absolute top-0 end-0 inline-flex items-center size-3.5 rounded-full border-2 border-white text-xs font-medium transition-all transform -translate-y-1/2 translate-x-1/2 bg-yellow-500 dark:border-slate-900 badge-container">
                            <span class="sr-only text-black badge-content transition-all transform ">
                                "¡En Progreso!"
                            </span>
                        </span>
                        <h1 class="font-alfa-slab text-xl sm:text-2xl lg:text-3xl text-center mb-5">
                            "Rust para C#/.NET Developers"
                        </h1>
                        <p class="container mx-auto">
                            "La guía esta hecha por la misma Microsoft y es para  desarrolladores experimentados en C#/.NET que exploran Rust. Ofrece una breve comparación, enlaces a recursos y respuestas rápidas."
                        </p>
                        <div class="mx-auto">
                            <ButtonLink
                                href="https://rustlanges.github.io/rust-para-dotnet-devs"
                                size="big"
                            >
                                "Ir a la guía"
                            </ButtonLink>
                        </div>
                    </div>
                </section>
            </div>

            <div class="container mx-auto mb-5">
                <h1 class="font-alfa-slab text-3xl sm:text-4xl lg:text-5xl text-center mb-5">
                    "¡Otros Libros!"
                </h1>
                <p class="max-w-screen-xl mx-auto text-center text-balance text-lg">
                    "Estos son algunos otros libros que nos interesa compartir con ustedes."
                </p>
            </div>
            <div class="flex flex-row justify-center items-center  container mx-auto">
                <section class="w-1/2 px-10">
                    <div class="relative group flex flex-col gap-y-6 border border-black p-2 sm:p-6 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] justify-between group transition-all transform">
                        <h1 class="font-alfa-slab text-xl sm:text-2xl lg:text-3xl text-center mb-5">
                            "Comprehensive Rust"
                        </h1>
                        <p class="container mx-auto">
                            "Curso gratuito y de código abierto desarrollado por el equipo de Android de Google. Cubre todos los aspectos de Rust, desde la sintaxis básica hasta temas avanzados. Se ven algunos temas especializados como Android, Chromium y Bare-metal."
                        </p>
                        <div class="mx-auto">
                            <ButtonLink
                                href="https://google.github.io/comprehensive-rust/es/index.html"
                                size="big"
                            >
                                "Ir a Comprehensive Rust"
                            </ButtonLink>
                        </div>
                    </div>
                </section>
            </div>
        </section>
    }
}