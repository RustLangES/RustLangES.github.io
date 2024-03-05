use leptos::{component, view, IntoView};

use crate::components::ButtonLink;

#[derive(Clone, Debug)]
struct BookData {
    name: &'static str,
    description: &'static str,
    english: bool,
    complete: bool,
    url: &'static str,
    url_name: &'static str,
}

const OTHER_BOOKS: [BookData; 5] = [BookData {
        name: "Comprehensive Rust",
        description: "Curso gratuito y de código abierto desarrollado por el equipo de Android de Google. Cubre todos los aspectos de Rust, desde la sintaxis básica hasta temas avanzados. Se ven algunos temas especializados como Android, Chromium y Bare-metal.",
        english: false,
        complete: true,
        url: "https://google.github.io/comprehensive-rust/es/index.html",
        url_name: "Ir a Comprehensive Rust",
    }, BookData { name: "Libro de Referencia de Rust", description: "Este libro es la guía definitiva para dominar Rust, explicando construcciones, memoria, concurrencia y más, con apéndices y referencias. Ideal para sumergirse en la programación con Rust.", english: true, complete: false, url: "https://doc.rust-lang.org/reference/introduction.html", url_name: "Ir a “El Libro de Referencia”" }, BookData { name: "Embedded Book", description: "Guía para usar Rust en sistemas integrados 'Bare Metal', ideal para programación integrada con seguridad y conceptos avanzados. Cubre configuración, prácticas y manuales, enfocado en ARM Cortex-M, sin asumir conocimientos previos.", english: true, complete: true, url: "https://docs.rust-embedded.org/book/intro/index.html", url_name: "Ir a “Embedded Book”" },
BookData {
    name: "The Rustonomicon",
    description: "¡Descubre los oscuros secretos de Rust no seguro! Este libro te lleva a las profundidades de la programación no segura en Rust, con detalles espeluznantes y útiles sobre su uso. Perfecto para aquellos que desean explorar las entrañas del lenguaje o escribir código no seguro. ¡Prepárate para una inmersión intensa en el lado oscuro de Rust!",
    english: true,
    complete: false,
    url: "https://doc.rust-lang.org/nomicon/intro.html",
    url_name: "Ir a “The Rustonomicon”",
},
BookData {
    name: "CXX - Safe Interop",
    description: "CXX facilita la integración segura entre Rust y C++, evitando errores comunes al llamar código de uno al otro. Con análisis estático y generadores de código, protege las invariantes de ambos lenguajes, asegurando una integración eficiente y correcta. ",
    english: true,
    complete: true,
    url: "https://cxx.rs/",
    url_name: "Ir a “CXX”",
}];

#[component]
pub fn Books() -> impl IntoView {
    let book = |book: BookData| {
        view! {
            <section class="w-full md:w-1/2 lg:w-1/3 h-full xs:px-8">
                <div class="relative group flex flex-col gap-y-6 border border-black p-2 sm:p-6 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] justify-between group transition-all transform">
                    {book.english.then_some(|| view! {
                        <span class="absolute top-0 end-0 inline-flex items-center size-3.5 group-hover:min-w-28 rounded-full border-2 border-white text-xs font-medium transition-all transform -translate-y-1/2 translate-x-1/2 bg-blue-400 dark:border-slate-900 badge-container">
                            <span class="sr-only text-black badge-content transition-all transform">
                                "En Inglés"
                            </span>
                        </span>
                    })}
                    <h1 class="font-alfa-slab text-xl sm:text-2xl lg:text-3xl text-center mb-5">
                        {book.name}
                    </h1>
                    <p class="container mx-auto">{book.description}</p>
                    {(!book.complete).then_some(||
                        view! {
                            <div class="flex gap-2 items-center bg-orange-200 rounded-md px-2 py-3">
                                <p class="font-work-sans text-black text-sm">"ℹ️ Este Libro está marcado como incompleto"</p>
                            </div>
                        }
                    )}
                    <div class="mx-auto">
                        <ButtonLink
                            href=book.url
                            size="big"
                            class="max-w-fit"
                        >
                        {book.url_name}
                        </ButtonLink>
                    </div>
                </div>
            </section>
        }
    };

    let books_list = move || {
        OTHER_BOOKS
            .into_iter()
            .map(book)
            .collect::<Vec<_>>()
            .into_view()
    };

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
            <div class="flex flex-col md:flex-row justify-center items-center container mx-auto mb-16 md:mb-28 gap-4">
                <section class="w-full md:w-1/2 px-8">
                    <div class="relative group flex flex-col gap-y-6 border border-black p-2 sm:p-6 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] justify-between group transition-all transform">
                        <span class="absolute top-0 end-0 inline-flex items-center size-3.5 group-hover:min-w-28 rounded-full border-2 border-white text-xs font-medium transition-all transform -translate-y-1/2 translate-x-1/2 bg-teal-500 dark:border-slate-900 badge-container">
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
                            <ButtonLink href="https://rustlang-es.org/rust-book-es" size="big">
                                "Ir a “El Libro”"
                            </ButtonLink>
                        </div>
                    </div>
                </section>

                <section class="w-full md:w-1/2 px-8 flex flex-col h-full">
                    <div class="group flex flex-col gap-y-6 border border-black p-2 sm:p-6 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between">
                        <span class="absolute top-0 end-0 inline-flex items-center size-3.5 group-hover:min-w-28 rounded-full border-2 border-white text-xs font-medium transition-all transform -translate-y-1/2 translate-x-1/2 bg-yellow-500 dark:border-slate-900 badge-container">
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
                                href="https://rustlang-es.org/rust-para-dotnet-devs"
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
                    "Estos son algunos otros libros que nos interesa compartir."
                </p>
            </div>
            <div class="flex flex-row flex-wrap w-full gap-y-6 justify-center items-center container mx-auto">
            {books_list}
            </div>
        </section>
    }
}
