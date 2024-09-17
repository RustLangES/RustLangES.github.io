use leptos::{component, view, IntoView};

use crate::components::{
    button_link::ButtonLink,
    button_large_link::ButtonLargeLink,
    icons::{DiscordIcon, CalendarIcon, GithubIcon},
};

#[component]
pub fn Hacktoberfest() -> impl IntoView {
    view! {
        <section class="bg-orange-200/30 dark:bg-transparent">
            <div class="container mx-auto px-4">
                <div class="flex flex-col items-center py-20 gap-y-6">
                    <h2 class="text-4xl text-center mb-4">
                        <span class="font-work-sans font-light">
                            "🚀 ¡Únete a la celebración del" 
                            <span class="font-alfa-slab text-orange-500">" Hacktoberfest "</span>
                            "con nosotros! 🚀"
                        </span>
                    </h2>
                    <p class="text-center text-xl">
                        "Este" <span class="text-orange-500 font-alfa-slab">" 5 de Octubre"</span>"
                        , nuestra comunidad se une a este emocionante evento de programación. 
                        Aprovecha esta excelente oportunidad para contribuir a proyectos open-source, aprender nuevas habilidades 
                        y conectar con otros amantes del open-source."
                    </p>
                    <div class="flex items-center gap-x-12 gap-y-6 flex-col *:w-full sm:flex-row">
                        <ButtonLink
                            href="https://discord.gg/4ng5HgmaMg"
                            shadow="box"
                            color="white"
                            size="big"
                        >
                            <DiscordIcon size=30/>
                            "Participa"
                        </ButtonLink>
                        <ButtonLargeLink
                            href="https://calendar.google.com/calendar/event?action=TEMPLATE&tmeid=M2ZiMjhzbGNqbTZoMjNrZ2ZpbW4zYzk1ZGUgZGFmYzYyODQwMzRkOWJlZjNlMzFkZTNiZDE1OTI2OGQ5OGU4YzlhOGY2MjU3Mzk4NGI3MGE1OWQ2NjU3ZjVhZkBn&tmsrc=dafc6284034d9bef3e31de3bd159268d98e8c9a8f62573984b70a59d6657f5af%40group.calendar.google.com"
                            shadow="box"
                            color="white"
                            size="big"
                        >
                            <CalendarIcon size=30/>
                            "Añade a tu calendario"
                        </ButtonLargeLink>
                        <ButtonLargeLink
                            href="https://forms.rustlang-es.org/form/1"
                            shadow="box"
                            color="white"
                            size="big"
                        >
                            <GithubIcon size=30/>
                            "Postula tu proyecto"
                        </ButtonLargeLink>
                    </div>
                </div>
            </div>
        </section>
    }
}
