use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant as ButtonVariant},
    card::Card,
};

use crate::components::{
    ad_section::AdSection, next_events::NextEvents, on_site_events::OnSiteEvents,
    stay_up_date::StayUpDate,
};

#[component]
pub fn Events() -> impl IntoView {
    view! {
        <div class="w-full h-[65dvh] rustlang-es-background-secondary dark:bg-[#3E1C96CC] flex items-center justify-center">
            <div class="w-full container  flex  flex-col-reverse md:flex-row items-center justify-between m-auto">
                <div class="flex flex-col justify-center gap-8">
                    <div class="flex flex-col gap-2 justify-center not-md:items-center w-full text-center lg:text-left">
                        <p class="uppercase text-h1 text-akira">Eventos</p>
                        <p class="text-heading-2 font-heading-4">
                            "Entérate de todas las actividades que tenemos en la comunidad"
                        </p>
                    </div>
                    <div class="flex gap-4 flex-wrap w-full not-md:justify-center not-md:items-center">
                        <Button
                            variant=ButtonVariant::Secondary
                            on_click=|_| {}
                            class="text-heading-4 font-heading-4  font-semibold"
                            label="Suscríbete"
                        />
                    </div>
                </div>
                <div>
                    <Card class="relative overflow-clip min-h-80 flex flex-col justify-center max-w-80">
                        <div class="absolute top-0 left-0 bg-black text-white w-full p-5">
                            <p class="uppercase w-full text-center text-akira">
                                "¡Últimos lugares!"
                            </p>
                        </div>
                        <div class="min-h-16"></div>
                        <div class="container flex flex-col justify-between items-center gap-5">
                            <div class="items-center justify-center flex flex-col gap-2.5">
                                <p class="uppercase text-heading-2 text-akira">"Hacktoberfest"</p>
                                <p class="font-thin text-akira">"5 de Octubre"</p>
                            </div>
                            <p class="font-heading-4 text-balance text-center">
                                "¡Nos unimos a este emocionante evento de programación!"
                            </p>
                            <Button
                                variant=ButtonVariant::Primary
                                on_click=|_| {}
                                class="text-heading-4 font-heading-4 font-semibold"
                                label="Comprar tickets"
                            />
                        </div>
                    </Card>
                </div>
            </div>
        </div>
        <NextEvents />
        <AdSection />
        <OnSiteEvents />
        <StayUpDate />
    }
}
