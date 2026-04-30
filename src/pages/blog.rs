use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant as ButtonVariant},
    card::Card,
    chip::{Chip, Variant as ChipVariant},
    icons::Filter,
};

use crate::components::footer::Footer;

#[derive(Clone)]
struct BlogPost {
    title: &'static str,
    date: &'static str,
    excerpt: &'static str,
    tags: &'static [&'static str],
    featured: bool,
    has_image: bool,
    num: &'static str,
}

const SPOTLIGHT_TAGS: &[&str] = &["Rust", "Comunidad", "Comunidad"];

const POSTS: &[BlogPost] = &[
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad", "Comunidad"], featured: true,  has_image: true,  num: "Num" },
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad", "Comunidad"], featured: true,  has_image: false, num: "Num" },
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad", "Comunidad"], featured: true,  has_image: false, num: "Num" },
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad", "Comunidad"], featured: true,  has_image: false, num: "Num" },
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad"],              featured: false, has_image: false, num: "Num" },
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad"],              featured: false, has_image: true,  num: "Num" },
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad"],              featured: false, has_image: false, num: "Num" },
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad"],              featured: false, has_image: true,  num: "Num" },
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad"],              featured: false, has_image: false, num: "Num" },
    BlogPost { title: "Esta semana en Rust", date: "1 de Enero 2025", excerpt: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.", tags: &["Rust", "Comunidad"],              featured: false, has_image: false, num: "Num" },
];

#[component]
fn PostTags(tags: &'static [&'static str]) -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            {tags.iter().map(|tag| view! {
                <Chip label=tag.to_string() variant=ChipVariant::Description />
            }).collect_view()}
        </div>
    }
}

#[component]
fn FeaturedPostCard(post: BlogPost, #[prop(default = false)] large: bool) -> impl IntoView {
    view! {
        <Card class="flex flex-col gap-0 overflow-hidden p-0">
            {large.then(|| view! {
                <div class="bg-neutral-900 h-48 w-full flex items-center justify-center relative overflow-hidden">
                    <div class="absolute inset-0 bg-gradient-to-b from-primary-500/30 to-neutral-900/70"></div>
                    <p class="text-akira text-white text-xl font-bold uppercase text-center z-10 px-4 leading-tight">
                        "ESTA SEMANA"
                        <br />
                        <span class="text-primary-400">"en RustLang ES"</span>
                    </p>
                </div>
            })}
            <div class="flex flex-col gap-3 p-5 flex-1">
                <div class="flex items-start justify-between gap-2">
                    <div class="flex flex-col gap-1">
                        <Chip label="Destacado" variant=ChipVariant::Featured />
                        <h3 class="font-bold text-base mt-1">{post.title}</h3>
                        <p class="text-xs text-neutral-500 dark:text-neutral-400 font-body">{post.date}</p>
                    </div>
                    <Chip label=post.num.to_string() variant=ChipVariant::Numeric />
                </div>
                <p class="text-sm text-neutral-600 dark:text-neutral-400 font-body leading-relaxed line-clamp-3">
                    {post.excerpt}
                </p>
                <PostTags tags=post.tags />
            </div>
        </Card>
    }
}

#[component]
fn RegularPostCard(post: BlogPost) -> impl IntoView {
    let has_image = post.has_image;
    let card_class = if has_image {
        "flex flex-row gap-4 p-5 items-start"
    } else {
        "flex flex-col gap-3 p-5"
    };

    view! {
        <Card class=card_class>
            <div class="flex flex-col gap-2 flex-1 min-w-0">
                <div class="flex items-start justify-between gap-2">
                    <div class="flex-1 min-w-0">
                        <h3 class="font-bold text-base leading-snug">{post.title}</h3>
                        <p class="text-xs text-neutral-500 dark:text-neutral-400 font-body mt-0.5">{post.date}</p>
                    </div>
                    <Chip label=post.num.to_string() variant=ChipVariant::Numeric />
                </div>
                <p class="text-sm text-neutral-600 dark:text-neutral-400 font-body leading-relaxed line-clamp-3">
                    {post.excerpt}
                </p>
                <PostTags tags=post.tags />
            </div>
            {has_image.then(|| view! {
                <div class="bg-orange-100 dark:bg-neutral-700 rounded-xl w-24 h-24 flex-shrink-0 overflow-hidden self-start">
                    <div class="w-full h-full bg-primary-500/10 flex items-center justify-center">
                        <img src="/assets/new/logos/logo-minified.svg" alt="" class="w-10 opacity-20" />
                    </div>
                </div>
            })}
        </Card>
    }
}

#[component]
pub fn Blog() -> impl IntoView {
    let featured_posts: Vec<BlogPost> = POSTS.iter().filter(|p| p.featured).cloned().collect();
    let regular_posts: Vec<BlogPost> = POSTS.iter().filter(|p| !p.featured).cloned().collect();

    view! {
        // Hero
        <div class="w-full min-h-[45dvh] rustlang-es-background dark:bg-[#F04906] text-akira flex items-center justify-center py-16 px-6">
            <div class="container max-w-7xl mx-auto flex flex-col md:flex-row items-center justify-between gap-8 lg:gap-16">
                <div class="flex flex-col gap-4 text-center md:text-left max-w-lg">
                    <h1 class="uppercase leading-tight">"BLOG"</h1>
                    <h2 class="font-bold font-body text-xl lg:text-2xl leading-snug">
                        "Revisa qué está pasando en la comunidad de Rust Lang en Español"
                    </h2>
                    <p class="text-sm font-body leading-relaxed opacity-80">
                        "Entérate de las últimas noticias, tutoriales, guías y todo lo que se nos ocurra sobre Rust."
                    </p>
                </div>
                <div class="flex-shrink-0">
                    <img
                        src="/assets/new/logos/ferris-hero.png"
                        alt="Ferris Blog"
                        class="w-48 lg:w-72"
                    />
                </div>
            </div>
        </div>

        // Spotlight featured post
        <section class="bg-white dark:bg-neutral-900 pt-12 pb-0 w-full">
            <div class="container max-w-7xl mx-auto px-6">
                <Card class="bg-secondary-50 dark:bg-secondary-950 border-secondary-200 dark:border-secondary-800 p-6 flex flex-col gap-3">
                    <div class="flex items-start justify-between gap-4 flex-wrap">
                        <div class="flex flex-col gap-2 flex-1">
                            <Chip label="Destacado" variant=ChipVariant::Featured />
                            <h2 class="font-bold text-2xl">"Bienvenidos a Rust Lang en Español"</h2>
                            <p class="text-xs text-neutral-500 dark:text-neutral-400 font-body">"1 de Enero 2025"</p>
                            <p class="font-body leading-relaxed text-neutral-700 dark:text-neutral-300 mt-1">
                                "Empieza por aquí."
                                <br />
                                "Este es el primer post de la comunidad de Rust en Español."
                            </p>
                        </div>
                        <Chip label="Num".to_string() variant=ChipVariant::Numeric />
                    </div>
                    <PostTags tags=SPOTLIGHT_TAGS />
                </Card>
            </div>
        </section>

        // Artículos
        <section class="bg-white dark:bg-neutral-900 py-16 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col gap-10">
                <h2 class="text-h2 text-center">"Artículos"</h2>

                // Destacados subsection
                <div class="flex flex-col gap-4">
                    <p class="text-sm text-neutral-500 dark:text-neutral-400 font-body font-medium">"Destacados"</p>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {featured_posts.iter().take(1).map(|p| {
                            let p = p.clone();
                            view! { <FeaturedPostCard post=p large=true /> }
                        }).collect_view()}
                        <div class="flex flex-col gap-4">
                            {featured_posts.iter().skip(1).take(2).map(|p| {
                                let p = p.clone();
                                view! { <FeaturedPostCard post=p /> }
                            }).collect_view()}
                        </div>
                        {featured_posts.iter().skip(3).map(|p| {
                            let p = p.clone();
                            view! { <FeaturedPostCard post=p /> }
                        }).collect_view()}
                    </div>
                </div>

                // Todos subsection
                <div class="flex flex-col gap-4">
                    <div class="flex items-center justify-between">
                        <p class="text-sm text-neutral-500 dark:text-neutral-400 font-body font-medium">"Todos"</p>
                        <div class="flex items-center gap-2">
                            <button class="p-1.5 rounded-lg border border-neutral-200 dark:border-neutral-700 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                    <polyline points="15 18 9 12 15 6"/>
                                </svg>
                            </button>
                            <span class="text-sm font-body px-1">"1"</span>
                            <button class="p-1.5 rounded-lg border border-neutral-200 dark:border-neutral-700 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors">
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                    <polyline points="9 18 15 12 9 6"/>
                                </svg>
                            </button>
                        </div>
                    </div>

                    // Search bar
                    <div class="flex gap-2 w-full">
                        <div class="flex-1 flex items-center gap-3 border-2 border-black dark:border-neutral-700 rounded-xl px-4 py-3 bg-white dark:bg-neutral-900 focus-within:border-primary-500 transition-colors">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-neutral-400 flex-shrink-0">
                                <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
                            </svg>
                            <input
                                type="text"
                                placeholder="Buscar"
                                class="flex-1 bg-transparent outline-none text-sm font-body placeholder-neutral-400 dark:placeholder-neutral-500"
                            />
                        </div>
                        <button class="border-2 border-black dark:border-neutral-700 rounded-xl px-4 py-3 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors flex-shrink-0 flex items-center">
                            <Filter />
                        </button>
                    </div>

                    // Posts grid
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {regular_posts.into_iter().map(|post| view! {
                            <RegularPostCard post=post />
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </section>

        <Footer />
    }
}
