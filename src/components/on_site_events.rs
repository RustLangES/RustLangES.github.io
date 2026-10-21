use super::photo_card::PhotoCard;
use leptos::prelude::*;

#[component]
pub fn OnSiteEvents() -> impl IntoView {
    view! {
        <section class="w-full py-20 dark:bg-[#333333] flex flex-col items-center gap-16 overflow-x-clip">
            <div class="container max-w-6xl px-6 flex flex-col md:flex-row items-center justify-between gap-16">
                <div class="flex flex-col gap-6 max-w-md text-center md:text-left">
                    <h2 class="text-heading-2 font-heading-2 text-3xl font-bold">
                        "Eventos presenciales"
                    </h2>
                    <p class="font-heading-4 text-lg leading-loose">
                        "Contamos con participación en eventos presenciales, tanto organizados como invitaciones"
                    </p>
                </div>
                <div class="relative w-full max-w-lg aspect-square">
                    <PhotoCard
                        src="/assets/presencial.png"
                        description="Descripción"
                        class="absolute right-0 bottom-0 w-3/5 -rotate-8"
                    />
                    <PhotoCard
                        src="/assets/presencial.png"
                        description="Descripción"
                        class="absolute left-0 top-0 w-3/5 rotate-20 z-10"
                    />
                </div>
            </div>
            <div class="container max-w-6xl px-6 flex flex-col items-center gap-8">
                <h2 class="text-heading-2 font-heading-2 text-3xl font-semibold text-center">
                    "Ediciones pasadas"
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 w-full">
                    <PhotoCard src="/assets/presencial.png" description="Descripción" />
                    <PhotoCard src="/assets/presencial.png" description="Descripción" />
                    <PhotoCard src="/assets/presencial.png" description="Descripción" />
                </div>
            </div>
        </section>
    }
}
