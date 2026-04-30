use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant as ButtonVariant},
    card::Card,
    icons::Discord,
};

use crate::components::footer::Footer;

struct UpcomingEvent {
    title: &'static str,
    date: &'static str,
    price: Option<&'static str>,
    description: &'static str,
    header_class: &'static str,
    cta: &'static str,
}

#[component]
pub fn Events() -> impl IntoView {
    let upcoming_events = vec![
        UpcomingEvent {
            title: "Hacktoberfest",
            date: "31 de octubre 2025",
            price: None,
            description: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.",
            header_class: "bg-primary-500",
            cta: "Inscribirse",
        },
        UpcomingEvent {
            title: "Masterclass",
            date: "31 de octubre 2025",
            price: Some("20 USD"),
            description: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.",
            header_class: "bg-orange-400",
            cta: "Comprar tickets",
        },
        UpcomingEvent {
            title: "Halloween en Rust",
            date: "3 de octubre 2025",
            price: None,
            description: "Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.",
            header_class: "bg-neutral-900",
            cta: "Inscribirse",
        },
    ];

    view! {
        // Hero
        <div class="w-full min-h-[65dvh] rustlang-es-background-secondary dark:bg-[#3E1C96CC] text-akira flex items-center justify-center py-16 px-6">
            <div class="container max-w-7xl mx-auto flex flex-col md:flex-row items-center justify-between gap-8 lg:gap-16">
                <div class="flex flex-col gap-6 text-center md:text-left">
                    <div class="flex flex-col gap-2">
                        <h1 class="uppercase leading-tight">"EVENTOS"</h1>
                        <p class="text-base font-normal font-body mt-2 leading-relaxed max-w-md">
                            "Entérate de todas las actividades que tenemos en la comunidad"
                        </p>
                    </div>
                    <div class="flex justify-center md:justify-start">
                        <Button
                            variant=ButtonVariant::Secondary
                            on_click=|_| {}
                            label="Suscríbete"
                        />
                    </div>
                </div>
                // Featured event card
                <div class="bg-neutral-950 text-white rounded-2xl p-6 w-full max-w-xs border border-neutral-800 flex-shrink-0">
                    <div class="bg-primary-500 text-white text-xs font-bold font-body px-3 py-1 rounded-full inline-block mb-4 uppercase tracking-wide">
                        "¡Últimos lugares!"
                    </div>
                    <h2 class="uppercase text-2xl font-bold leading-tight mb-3">
                        "HACKTOBERFEST"
                        <br />
                        "5 DE OCTUBRE"
                    </h2>
                    <p class="text-sm text-neutral-300 mb-6 font-body leading-relaxed">
                        "¡Nos unimos a este emocionante evento de programación!"
                    </p>
                    <Button
                        variant=ButtonVariant::Primary
                        on_click=|_| {}
                        label="Comprar tickets"
                    />
                </div>
            </div>
        </div>

        // Próximos eventos
        <section class="bg-white dark:bg-neutral-900 py-20 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col items-center gap-8">
                <h2 class="text-h2 text-center">"Próximos eventos"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 w-full">
                    {upcoming_events.into_iter().map(|ev| {
                        let header_class = format!("{} h-40 w-full flex items-center justify-center px-4", ev.header_class);
                        view! {
                            <div class="border-2 border-black dark:border-neutral-700 rounded-2xl overflow-hidden flex flex-col drop-shadow-cards bg-white dark:bg-neutral-900">
                                <div class=header_class>
                                    <span class="text-white font-bold text-xl text-center text-akira uppercase">
                                        {ev.title}
                                    </span>
                                </div>
                                <div class="p-5 flex flex-col gap-3 flex-1">
                                    <div class="flex items-start justify-between gap-2">
                                        <div>
                                            <h3 class="font-bold text-base">{ev.title}</h3>
                                            <p class="text-sm text-neutral-500 dark:text-neutral-400 font-body">{ev.date}</p>
                                        </div>
                                        <span class="text-xs bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-300 border border-orange-300 dark:border-orange-800 px-2 py-0.5 rounded-full whitespace-nowrap font-body flex-shrink-0">
                                            {ev.price.unwrap_or("Gratis")}
                                        </span>
                                    </div>
                                    <p class="text-sm text-neutral-600 dark:text-neutral-400 font-body leading-relaxed flex-1">
                                        {ev.description}
                                    </p>
                                    <Button
                                        variant=ButtonVariant::Primary
                                        on_click=|_| {}
                                        label=ev.cta
                                    />
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>

        // Tu publicidad + Nuestra comunidad
        <section class="bg-orange-50 dark:bg-neutral-950 py-16 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col items-center gap-4 text-center max-w-lg mx-auto">
                <h2 class="text-h2">
                    <span class="text-primary-500 dark:text-orange-300">"Tu publicidad"</span>
                    " + Nuestra comunidad"
                </h2>
                <p class="font-body leading-relaxed text-neutral-700 dark:text-neutral-300 max-w-md">
                    "¿Sabías que puedes publicitar tus productos o servicios en nuestros eventos y página web?"
                </p>
                <p class="font-body text-neutral-600 dark:text-neutral-400 max-w-md">
                    "Ponte en contacto con nosotros para conocer todos los beneficios."
                </p>
                <Button
                    variant=ButtonVariant::Secondary
                    on_click=|_| {}
                    label="¡Quiero publicitar!"
                />
            </div>
        </section>

        // Eventos presenciales
        <section class="bg-white dark:bg-neutral-900 py-20 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col md:flex-row items-center gap-12">
                <div class="flex flex-col gap-4 max-w-sm">
                    <h2 class="text-h2">"Eventos presenciales"</h2>
                    <p class="font-body text-neutral-600 dark:text-neutral-400 leading-relaxed">
                        "Contamos con participación en eventos presenciales, tanto organizados como invitaciones"
                    </p>
                </div>
                <div class="flex-1 flex items-center justify-center relative h-72 min-w-64">
                    <div class="absolute" style="transform: rotate(-6deg) translate(-24px, 12px);">
                        <div class="bg-orange-50 dark:bg-neutral-800 border-2 border-neutral-200 dark:border-neutral-700 rounded-sm p-3 shadow-xl w-44 h-56 flex flex-col">
                            <div class="flex-1 bg-orange-100 dark:bg-neutral-700 rounded-sm"></div>
                            <p class="text-center text-xs mt-2 font-body text-neutral-500">"Descripción"</p>
                        </div>
                    </div>
                    <div class="absolute" style="transform: rotate(5deg) translate(32px, -8px);">
                        <div class="bg-orange-50 dark:bg-neutral-800 border-2 border-neutral-200 dark:border-neutral-700 rounded-sm p-3 shadow-xl w-44 h-56 flex flex-col">
                            <div class="flex-1 bg-orange-100 dark:bg-neutral-700 rounded-sm"></div>
                            <p class="text-center text-xs mt-2 font-body text-neutral-500">"Descripción"</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // Ediciones pasadas
        <section class="bg-neutral-50 dark:bg-neutral-950 py-20 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col items-center gap-8">
                <h2 class="text-h2 text-center">"Ediciones pasadas"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-6 w-full">
                    {(0..3u8).map(|_| view! {
                        <Card class="overflow-hidden p-0">
                            <div class="bg-orange-100 dark:bg-neutral-800 h-44 w-full"></div>
                            <p class="text-center text-sm p-3 font-body text-neutral-600 dark:text-neutral-400">
                                "Descripción"
                            </p>
                        </Card>
                    }).collect_view()}
                </div>
            </div>
        </section>

        // Mantente al día
        <section class="bg-primary-500 dark:bg-primary-600 py-16 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col md:flex-row items-center justify-between gap-8">
                <div class="flex flex-col gap-4 max-w-md text-center md:text-left">
                    <h2 class="text-h2 text-akira">"Mantente al día"</h2>
                    <p class="font-body leading-relaxed">
                        "Entérate de todos los "
                        <span class="font-bold">"eventos"</span>
                        " y "
                        <span class="font-bold">"actividades en línea"</span>
                        " en los que puedes participar en nuestro Discord"
                    </p>
                    <div class="flex justify-center md:justify-start">
                        <Button
                            variant=ButtonVariant::Secondary
                            on_click=|_| {}
                            label="¡Únete!"
                            icon=view! { <Discord /> }.into_any()
                        />
                    </div>
                </div>
                <div class="flex-shrink-0">
                    <img
                        src="/assets/new/logos/ferris-hero.png"
                        alt="Ferris en Discord"
                        class="w-48 lg:w-64"
                    />
                </div>
            </div>
        </section>

        // Agenda 2025
        <section class="bg-white dark:bg-neutral-900 py-20 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col md:flex-row items-center gap-12">
                <div class="flex flex-col gap-4 max-w-sm">
                    <h2 class="text-h2">"Agenda 2025"</h2>
                    <p class="font-body text-neutral-600 dark:text-neutral-400 leading-relaxed">
                        "Revisa nuestro calendario y anticípate a reservar tu lugar"
                    </p>
                </div>
                <div class="flex-1 flex flex-col sm:flex-row gap-6 items-start w-full">
                    // Calendar widget
                    <div class="border-2 border-black dark:border-neutral-700 rounded-2xl p-4 w-64 flex-shrink-0">
                        <div class="flex items-center justify-between mb-3">
                            <button class="text-sm px-2">"◁"</button>
                            <span class="font-bold font-body text-sm">"Febrero 2025"</span>
                            <button class="text-sm px-2">"▷"</button>
                        </div>
                        <div class="grid grid-cols-7 gap-0.5 text-xs font-body text-center mb-1">
                            {["Su","Mo","Tu","We","Th","Fr","Sa"].iter().map(|d| view! {
                                <span class="text-neutral-400 py-1 text-[10px]">{*d}</span>
                            }).collect_view()}
                        </div>
                        <div class="grid grid-cols-7 gap-0.5 text-xs font-body text-center">
                            // Feb 2025 starts on Saturday (day 6 = index 6)
                            {(0..6u8).map(|_| view! { <span></span> }).collect_view()}
                            {(1..=28u8).map(|d| view! {
                                <span class=if d == 20 {
                                    "bg-primary-500 text-white rounded-full w-6 h-6 flex items-center justify-center mx-auto font-bold cursor-pointer"
                                } else {
                                    "w-6 h-6 flex items-center justify-center mx-auto hover:bg-neutral-100 dark:hover:bg-neutral-800 rounded-full cursor-pointer"
                                }>{d}</span>
                            }).collect_view()}
                        </div>
                    </div>
                    // Event list
                    <div class="flex flex-col gap-3 flex-1 w-full">
                        <div class="flex items-center gap-3 bg-primary-500 text-white rounded-xl px-4 py-3 cursor-pointer hover:bg-primary-600 transition-colors">
                            <div class="w-2 h-2 rounded-full bg-white flex-shrink-0"></div>
                            <span class="text-sm font-body font-bold">"Masterclass RustLangES"</span>
                        </div>
                        <div class="flex items-center gap-3 bg-orange-200 dark:bg-orange-900/30 text-neutral-900 dark:text-orange-100 rounded-xl px-4 py-3 cursor-pointer hover:bg-orange-300 dark:hover:bg-orange-900/50 transition-colors">
                            <div class="w-2 h-2 rounded-full bg-primary-500 flex-shrink-0"></div>
                            <span class="text-sm font-body font-bold">"Conferencia :25"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <Footer />
    }
}
