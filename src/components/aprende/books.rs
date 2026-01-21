use leptos::{component, view, IntoView};
use std::collections::HashMap;
use leptos::prelude::*;

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
    description: "Explora los aspectos oscuros de la programación no segura en Rust con este libro, que ofrece detalles espeluznantes y prácticos para aquellos interesados en adentrarse en el código no seguro. Una inmersión intensa en el lado oscuro de Rust.",
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
            <Book
                title=book.name
                description=book.description
                link=book.url
                link_text=book.url_name
                incomplete=!book.complete
            >
                {book
                    .english
                    .then_some(|| {
                        view! { <Badge color="teal">"En Inglés"</Badge> }
                    })}

            </Book>
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
        <section class=" bg-orange-300/30 dark:bg-transparent py-20">
            <div class="container mx-auto mb-5">
                <h1 class="font-alfa-slab text-3xl sm:text-4xl lg:text-5xl text-center mb-5">
                    "¡Nuestros Libros!"
                </h1>
                <p class="max-w-screen-xl mx-auto text-center text-balance text-lg">
                    "Hemos dedicado tiempo y esfuerzo a adaptar libros al español, estamos orgullosos de compartir estos recursos, esperando que contribuyan al acceso y comprensión de valiosos conocimientos."
                </p>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 container mx-auto mb-16 md:mb-28 gap-4 h-fit">
                <Book
                    title="El lenguaje de Programación Rust"
                    description="Cariñosamente conocido como “el libro”, El Lenguaje de Programación Rust te dará una visión del lenguaje desde los principios básicos. Construirás unos cuantos proyectos por el camino y, al final, tendrás una comprensión sólida del lenguaje."
                    link="https://book.rustlang-es.org"
                    link_text="Ir a “El Libro”"
                >
                    <Badge color="teal">"Recomendado"</Badge>
                </Book>
                <Book
                    title="Rust para C#/.NET Developers"
                    description="La guía esta hecha por la misma Microsoft y es para  desarrolladores experimentados en C#/.NET que exploran Rust. Ofrece una breve comparación, enlaces a recursos y respuestas rápidas."
                    link="https://dotnet-book.rustlang-es.org"
                    link_text="Ir a la guía"
                >
                    <Badge color="teal">"Completo"</Badge>
                </Book>
            </div>

            <div class="container mx-auto mb-5">
                <h1 class="font-alfa-slab text-3xl sm:text-4xl lg:text-5xl text-center mb-5">
                    "¡Otros Libros!"
                </h1>
                <p class="max-w-screen-xl mx-auto text-center text-balance text-lg">
                    "Estos son algunos otros libros que nos interesa compartir."
                </p>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 w-full gap-y-6 container mx-auto">
                {books_list}
            </div>
        </section>
    }
}

#[component]
fn Book(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] link: String,
    #[prop(into)] link_text: String,
    #[prop(optional)] incomplete: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <article class="w-full h-full px-8">
            <div class="h-full relative group flex flex-col gap-y-6 border border-black p-2 sm:p-6 bg-orange-100  dark:hover:bg-zinc-900/40 dark:bg-black/40  drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] justify-between group transition-all transform">
                {children()}
                <h1 class="font-alfa-slab text-xl sm:text-2xl lg:text-3xl text-center mb-5">
                    {title}
                </h1> <p class="container mx-auto text-pretty">{description}</p>
                {incomplete
                    .then_some(|| {
                        view! {
                            <div class="flex gap-2 items-center bg-orange-200 rounded-md px-2 py-3">
                                <p class="font-work-sans text-black text-sm">
                                    "Este Libro está marcado como incompleto!"
                                </p>
                            </div>
                        }
                    })}
                <div class="mx-auto text-center text-sm font-bold pt-14 sm:text-sm md:text-base lg:text-lg leading-tight">
                    <ButtonLink href=link size="big" class="p-8">
                        {link_text}
                    </ButtonLink>
                </div>
            </div>
        </article>
    }
}

#[component]
fn Badge(color: &'static str, children: Children) -> impl IntoView {
    let colors = HashMap::from([("teal", "bg-teal-500"), ("yellow", "bg-yellow-500")]);
    let color = (*colors.get(&color).expect("Unknown color")).to_string();

    view! {
        <span class=format!(
            "absolute top-0 end-0 inline-flex items-center size-3.5 group-hover:min-w-28 rounded-full border-2 border-white text-xs font-medium transition-all transform -translate-y-1/2 translate-x-1/2 badge-container dark:border-slate-900 {}",
            color,
        )>
            <span class="sr-only text-black badge-content transition-all transform">
                {children()}
            </span>
        </span>
    }
}
