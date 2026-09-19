use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant as ButtonVariant},
    card::Card,
    chip::{Chip, Variant as ChipVariant},
    icons::Telegram,
    input::{Filter, InputSearch},
};

type ResourceAlias = Vec<Filter>;

#[allow(non_snake_case)]
pub fn OtherCommunities() -> impl IntoView {
    view! {
        <section class="bg-orange-100 dark:bg-neutral-950 p-4 lg:p-20 w-full">
            <div class="flex flex-col justify-center items-center w-full container xl:max-w-7xl lg:px-36 lg:pb-5 mx-auto">
                <h2 class="text-h2 text-center mb-4">"Otras Comunidades Rust"</h2>
                <div>
                    <p class="text-center max-w-lg">"Encuentra la comunidad más cercana a ti"</p>
                </div>
                <div class="flex flex-col justify-center mb-8 container mt-10 mx-auto max-w-fit">
                    // TODO: Implement functionality referenced in the issue: https://github.com/RustLangES/RustLangES.github.io/issues/113
                    <InputSearch on_change_filter=move |_resource: ResourceAlias| () />
                </div>
            </div>
            <div class="flex flex-row justify-center items-center gap-4 max-w-full m-auto">
                <div class="items-center gap-8 overflow-x-auto p-4 grid md:grid-cols-2 xl:grid-cols-3">

                    <div class="min-w-87.5 max-w-md h-full mt-5">
                        <Card class="min-w-87.5 max-w-md h-full p-4">
                            <div class="flex flex-col gap-4 justify-between h-full">
                                <div class="flex flex-col items-center gap-4  h-full justify-stretch p-2">
                                    <div class="relative">
                                        <Button
                                            class="-top-3 -right-2 absolute"
                                            icon=view! { <Telegram /> }.into_any()
                                            variant=ButtonVariant::Icon
                                            on_click=move |_| {}
                                        />
                                        <Avatar
                                            url="https://rustlang-es.org/gen_assets/rust-colombia.webp"
                                            size=100
                                        />
                                    </div>
                                    <h2 class="text-h3 mt-2">"Rust Colombia"</h2>
                                    <Chip label="Colombia" variant=ChipVariant::Location />
                                    <p class="text-paragraph-3 mb-4 text-center text-balance mx-auto">
                                        "Comunidad de Rust con dos sedes, en Medellín y en Bogotá"
                                    </p>
                                </div>
                            </div>
                        </Card>
                    </div>

                    <div class="min-w-87.5 max-w-md h-full mt-5">
                        <Card class="min-w-87.5 max-w-md h-full p-4">
                            <div class="flex flex-col gap-4 justify-between h-full">
                                <div class="flex flex-col items-center gap-4 h-full justify-stretch p-2">
                                    <div class="relative">
                                        <Button
                                            class="-top-3 -right-2 absolute"
                                            icon=view! { <Telegram /> }.into_any()
                                            variant=ButtonVariant::Icon
                                            on_click=move |_| {}
                                        />
                                        <Avatar
                                            url="https://rustlang-es.org/gen_assets/rust-venezuela.webp"
                                            size=100
                                        />
                                    </div>
                                    <h2 class="text-h3 mt-2">"Rust Venezuela"</h2>
                                    <Chip label="Venezuela duh" variant=ChipVariant::Location />
                                </div>
                            </div>
                        </Card>
                    </div>

                    <div class="min-w-87.5 max-w-md h-full mt-5">
                        <Card class="min-w-87.5 max-w-md h-full p-4">
                            <div class="flex flex-col gap-4 justify-between h-full">
                                <div class="flex flex-col items-center gap-4 h-full justify-stretch p-2">
                                    <div class="relative">
                                        <Button
                                            class="-top-3 -right-2 absolute"
                                            icon=view! { <Telegram /> }.into_any()
                                            variant=ButtonVariant::Icon
                                            on_click=move |_| {}
                                        />
                                        <Avatar
                                            url="https://rustlang-es.org/gen_assets/rust-argentina.webp"
                                            size=100
                                        />
                                    </div>
                                    <h2 class="text-h3 mt-2">"Rust Argentina"</h2>
                                    <Chip label="Argentina claro" variant=ChipVariant::Location />
                                    <p class="text-paragraph-3 mb-4 text-center text-balance mx-auto">
                                        "Anteriormente Rust Argentina, hoy una comunidad internacional"
                                    </p>
                                </div>
                            </div>
                        </Card>
                    </div>

                    <div class="min-w-87.5 max-w-md h-full mt-5">
                        <Card class="min-w-87.5 max-w-md h-full p-4">
                            <div class="flex flex-col gap-4 justify-between h-full">
                                <div class="flex flex-col items-center gap-4 h-full justify-stretch p-2">
                                    <div class="relative">
                                        <Button
                                            class="-top-3 -right-2 absolute"
                                            icon=view! { <Telegram /> }.into_any()
                                            variant=ButtonVariant::Icon
                                            on_click=move |_| {}
                                        />
                                        <Avatar
                                            url="https://rustlang-es.org/gen_assets/rust_ecuador.webp"
                                            size=100
                                        />
                                    </div>
                                    <h2 class="text-h3 mt-2">"Rust Ecuador"</h2>
                                    <Chip label="Ecuador" variant=ChipVariant::Location />
                                    <p class="text-paragraph-3 mb-4 text-center text-balance mx-auto">
                                        "¡Comunidad de Rust en Ecuador!"
                                    </p>
                                </div>
                            </div>
                        </Card>
                    </div>

                    <div class="min-w-87.5 max-w-md h-full mt-5">
                        <Card class="min-w-87.5 max-w-md h-full p-4">
                            <div class="flex flex-col gap-4 justify-between h-full">
                                <div class="flex flex-col items-center gap-4 h-full justify-stretch p-2">
                                    <div class="relative">
                                        <Button
                                            class="-top-3 -right-2 absolute"
                                            icon=view! { <Telegram /> }.into_any()
                                            variant=ButtonVariant::Icon
                                            on_click=move |_| {}
                                        />
                                        <Avatar
                                            url="https://rustlang-es.org/gen_assets/RustMX-min.webp"
                                            size=100
                                        />
                                    </div>
                                    <h2 class="text-h3 mt-2">"Rust MX"</h2>
                                    <Chip label="Mexico" variant=ChipVariant::Location />
                                    <p class="text-paragraph-3 mb-4 text-center text-balance mx-auto">
                                        "¡Comunidad de Rust en Mexico!"
                                    </p>
                                </div>
                            </div>
                        </Card>
                    </div>

                    <div class="min-w-87.5 max-w-md h-full mt-5">
                        <Card class="min-w-87.5 max-w-md h-full p-4">
                            <div class="flex flex-col gap-4 justify-between h-full">
                                <div class="flex flex-col items-center gap-4 h-full justify-stretch p-2">
                                    <div class="relative">
                                        <Button
                                            class="-top-3 -right-2 absolute"
                                            icon=view! { <Telegram /> }.into_any()
                                            variant=ButtonVariant::Icon
                                            on_click=move |_| {}
                                        />
                                        <Avatar
                                            url="https://rustlang-es.org/gen_assets/rust-peru.webp"
                                            size=100
                                        />
                                    </div>
                                    <h2 class="text-h3 mt-2">"Rust Perú"</h2>
                                    <Chip label="Perú" variant=ChipVariant::Location />
                                    <p class="text-paragraph-3 mb-4 text-center text-balance mx-auto">
                                        "¡Comunidad de Rust en Perú!"
                                    </p>
                                </div>
                            </div>
                        </Card>
                    </div>

                </div>
            </div>
        </section>
    }
}
