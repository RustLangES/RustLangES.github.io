use leptos::prelude::*;
use rustlanges_components::card::Card;

#[component]
pub fn OnSiteEvents() -> impl IntoView {
    view! {
        <div class="w-full h-[90dvh] dark:bg-neutral-900 flex flex-col items-center justify-center overflow-clip">
            <div class="w-full container flex flex-col-reverse md:flex-row items-center justify-between m-auto">
                <div class="flex flex-col justify-center gap-8 container max-w-md">
                    <div class="flex flex-col gap-2 justify-center not-md:items-center w-full text-center lg:text-left">
                        <p class="text-heading-2 font-heading-2 text-4xl font-bold">
                            "Eventos presenciales"
                        </p>
                        <p class="font-heading-3 text-lg">
                            "Contamos con participación en eventos presenciales, tanto organizados como invitaciones"
                        </p>
                    </div>
                </div>
                <div class="relative min-w-lg max-w-full flex flex-row justify-between items-start">
                    <Card class="relative max-w-full rotate-35 z-10">
                        <img
                            src="/assets/presencial.png"
                            class="shadow-brutal rounded-lg border border-black mb-2"
                        />
                        <p class="font-heading-4 text-balance text-center text-lg">
                            "Descripción"
                        </p>
                    </Card>

                    <Card class=" max-w-full -rotate-20 mt-20">
                        <img
                            src="/assets/presencial.png"
                            class="shadow-brutal rounded-lg border border-black mb-2"
                        />
                        <p class="font-heading-4 text-balance text-center text-lg">
                            "Descripción"
                        </p>
                    </Card>
                </div>
            </div>
            <div class="flex flex-col justify-center gap-8 mb-20">
                <div class="flex flex-col gap-2 justify-center not-md:items-center w-full text-center">
                    <p class="text-heading-2 font-heading-2 text-4xl font-bold text-center">
                        "Ediciones pasadas"
                    </p>
                    <div class="flex flex-row justify-center items-center gap-4  max-w-full flex-wrap">
                        <Card class="">
                            <img
                                src="/assets/presencial.png"
                                class="shadow-brutal rounded-lg border border-black mb-2"
                            />
                            <p class="font-heading-4 text-balance text-center text-lg">
                                "Descripción"
                            </p>
                        </Card>
                        <Card class="">
                            <img
                                src="/assets/presencial.png"
                                class="shadow-brutal rounded-lg border border-black mb-2"
                            />
                            <p class="font-heading-4 text-balance text-center text-lg">
                                "Descripción"
                            </p>
                        </Card>
                        <Card class="">
                            <img
                                src="/assets/presencial.png"
                                class="shadow-brutal rounded-lg border border-black mb-2"
                            />
                            <p class="font-heading-4 text-balance text-center text-lg">
                                "Descripción"
                            </p>
                        </Card>
                    </div>
                </div>
            </div>
        </div>
    }
}
