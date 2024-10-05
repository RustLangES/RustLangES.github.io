use leptos::{component, view, IntoView};

use crate::components::ButtonLink;

#[component]
pub fn Youtube() -> impl IntoView {
    view! {
        <section class="bg-orange-200 dark:bg-transparent py-20 gap-10 w-full lg:px-5">
            <div class="">
                <h1 class="font-alfa-slab text-2xl sm:text-3xl lg:text-4xl text-center text-balance mb-5">
                    "Algunos canales de Youtube que recomendamos"
                </h1>
                <p class="max-w-screen-xl mx-auto text-center text-balance text-lg">
                    "Explora Rust a través de nuestra selección de canales de YouTube recomendados por la comunidad. Desde tutoriales hasta análisis profundos, sumérgete en contenido de calidad sobre el lenguaje de programación Rust."
                </p>
            </div>
            <div class="p-6 w-full">
                <div class="list py-6">
                    <YoutubeCard
                        username="robertohuertasm"
                        title="Roberto Huertas"
                        description="Canal dedicado a la divulgación del lenguaje de programación Rust a través de vídeos explicativos y tutoriales."
                    />
                    <YoutubeCard
                        username="pineiden"
                        title="DAVID ALEJANDRO PINEDA"
                        description="Este canal es para compartir libremente mis conocimientos sobre tecnologías. Talleres, estudios, comentarios. Sobre todo herramientas que sean Software LIbre y temas relacionadas a la Cultura Libre."
                    />
                    <YoutubeCard
                        username="piny4man"
                        title="Piny4man"
                        description="Canal generalmente de resubidos, Piny4man suele stremear en Twitch contenido de Rust, desde backend con Axum y Actix a frontend con Leptos y Dioxus."
                        twitch=true
                    />
                    <YoutubeCard
                        username="dotmyself"
                        title="Dot Myself"
                        description="Lorem ipsum dolor sit amet consectetur adipisicing elit. Nam minima libero quidem qui minus magnam ut obcaecati, laborum quis ipsum dolores dolore aliquid architecto nisi fugiat pariatur earum quas eum."
                    />
                    <!-- "Esto es scrolleable así que se pueden agregar más" -->
                </div>
            </div>

        </section>
    }
}

#[component]
fn YoutubeCard(
    #[prop(into)] username: &'static str,
    #[prop(into)] title: &'static str,
    #[prop(into)] description: &'static str,
    #[prop(optional)] twitch: bool,
) -> impl IntoView {
    let assets_folder = if cfg!(debug_assertions) {
        "./assets/youtube"
    } else {
        "./youtube"
    };

    view! {
        <div class="list-container flex flex-col h-full gap-y-1 p-4 sm:p-6 bg-orange-100 dark:hover:bg-zinc-900/40 dark:bg-black/40  drop-shadow-[0_0_0_rgba(0,0,0)] border border-black justify-between mb-4">
            <img
                src=format!("{}/{}.avif", assets_folder, username)
                alt=format!("Foto del canal {}", username)
                class="size-32 rounded-full"
            />
            <h3 class="font-semibold">{title}</h3>
            <p>{description}</p>
            <div class="flex justify-center items-center gap-2">
                <ButtonLink
                    href=format!("https://www.youtube.com/@{}", username)
                    size="tiny"
                    class="p-2"
                >
                    "Youtube"
                </ButtonLink>
                {if twitch {
                    view! {
                        <>
                            <ButtonLink
                                href=format!("https://www.twitch.tv/{}", username)
                                size="tiny"
                                class="p-2"
                            >
                                "Twitch"
                            </ButtonLink>
                        </>
                    }
                } else {
                    view! { <>""</> }
                }}

            </div>

            {}
        </div>
    }
}
