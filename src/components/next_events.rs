use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant as ButtonVariant},
    card::Card,
    chip::{Chip, Variant as ChipVariant},
};

#[component]
pub fn NextEvents() -> impl IntoView {
    // TODO: Mover a un componente la parte repetitiva
    view! {
        <section class="flex flex-col justify-center items-center w-full gap-8 py-20 dark:bg-neutral-900">
            <div class="container flex flex-col justify-center items-center gap-8">
                <h2 class="text-h2 mb-4">"Próximos eventos"</h2>
            </div>

            <div class="flex flex-row justify-center items-center gap-4  max-w-full flex-wrap">
                {}
                <Card class="flex flex-col items-center justify-center relative overflow-clip p-0 max-w-sm">
                    <img
                        src="/assets/events_thumbnails/hacktoberfest.png"
                        class="w-full border-b-2 border-black"
                    />
                    <div class="p-10 gap-4 flex flex-col">
                        <div class="flex flex-row justify-between">
                            <div>
                                <h3 class="text-heading-4 font-heading-4 text-xl font-bold">
                                    "Hacktoberfest"
                                </h3>
                                <p class="font-caption text-caption-bold">31 de octubre 2025</p>
                            </div>
                            <div>
                                <Chip label="Gratis" variant=ChipVariant::Description />
                            </div>
                        </div>
                        <p>
                            Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.
                        </p>
                        <div class="flex w-full justify-end items-end">
                            <Button
                                variant=ButtonVariant::Primary
                                on_click=|_| {}
                                class="text-heading-4 font-heading-4 font-semibold"
                                label="Inscribirse"
                            />
                        </div>
                    </div>
                </Card>
                <Card class="flex flex-col items-center justify-center relative overflow-clip p-0 max-w-sm">
                    <img
                        src="/assets/events_thumbnails/masterclass.png"
                        class="w-full border-b-2 border-black"
                    />
                    <div class="p-10 gap-4 flex flex-col">
                        <div class="flex flex-row justify-between">
                            <div>
                                <h3 class="text-heading-4 font-heading-4 text-xl font-bold">
                                    "Masterclass"
                                </h3>
                                <p class="font-caption text-caption-bold">31 de octubre 2025</p>
                            </div>
                            <div>
                                <Chip
                                    label="20 USD"
                                    numeric_symbol=""
                                    variant=ChipVariant::Numeric
                                />
                            </div>
                        </div>
                        <p>
                            Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.
                        </p>
                        <div class="flex w-full justify-end items-end">
                            <Button
                                variant=ButtonVariant::Primary
                                on_click=|_| {}
                                class="text-heading-4 font-heading-4 font-semibold"
                                label="Comprar tickets"
                            />
                        </div>
                    </div>
                </Card>
                <Card class="flex flex-col items-center justify-center relative overflow-clip p-0 max-w-sm">
                    <img
                        src="/assets/events_thumbnails/halloween.png"
                        class="w-full border-b-2 border-black"
                    />
                    <div class="p-10 gap-4 flex flex-col">
                        <div class="flex flex-row justify-between">
                            <div>
                                <h3 class="text-heading-4 font-heading-4 text-xl font-bold">
                                    "Halloween en Rust"
                                </h3>
                                <p class="font-caption text-caption-bold">31 de octubre 2025</p>
                            </div>
                            <div>
                                <Chip label="Gratis" variant=ChipVariant::Description />
                            </div>
                        </div>
                        <p>
                            Lorem ipsum dolor sit amet consectetur. Integer commodo quisque et lobortis lacus ut.
                        </p>
                        <div class="flex w-full justify-end items-end">
                            <Button
                                variant=ButtonVariant::Primary
                                on_click=|_| {}
                                class="text-heading-4 font-heading-4 font-semibold"
                                label="Inscribirse"
                            />
                        </div>
                    </div>
                </Card>

            </div>
        </section>
    }
}
