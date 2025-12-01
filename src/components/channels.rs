use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant as ButtonVariant},
    card::Card,
    icons::Youtube,
};

pub fn Channels() -> impl IntoView {
    view! {
        <section class="bg-light dark:bg-dark p-4 lg:p-20 w-full">
            <div class="container xl:max-w-7xl lg:px-36 mx-auto">
                <h2 class="text-h2 text-center mb-8 mt-16">"Canales Recomendados"</h2>
            </div>

            <div class="flex flex-row justify-center items-center max-w-full w-full">
                <div class="inline-flex flex-row items-center justify-center gap-8 not-md:overflow-x-auto p-4 md:flex-wrap w-full">
                    <Card class="w-md">
                        <div class="flex flex-col gap-4 justify-between h-full">
                            <div class="flex flex-row items-center gap-4">
                                <Avatar
                                    url="https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4"
                                    size=52
                                />
                                <div>
                                    <h2 class="text-heading-3 mt-2 mb-2">"Nombre canal"</h2>
                                    <h3 class="text-paragraph-3 mt-2 mb-2">"Subtítulo"</h3>
                                </div>
                            </div>
                            <div>
                                <p class="text-paragraph-3 mb-4">
                                    "Breve descripción del canal o recurso contando qué muestra o para qué sirve."
                                </p>
                            </div>
                            <div class="flex items-end justify-end w-full gap-2 flex-wrap">
                                <Button
                                    variant=ButtonVariant::Icon
                                    on_click=|_| {}
                                    icon=view! { <Youtube /> }.into_any()
                                />
                            </div>
                        </div>
                    </Card>

                    <Card class="w-md">
                        <div class="flex flex-col gap-4 justify-between h-full">
                            <div class="flex flex-row items-center gap-4">
                                <Avatar
                                    url="https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4"
                                    size=52
                                />
                                <div>
                                    <h2 class="text-heading-3 mt-2 mb-2">"Nombre canal"</h2>
                                    <h3 class="text-paragraph-3 mt-2 mb-2">"Subtítulo"</h3>
                                </div>
                            </div>
                            <div>
                                <p class="text-paragraph-3 mb-4">
                                    "Breve descripción del canal o recurso contando qué muestra o para qué sirve."
                                </p>
                            </div>
                            <div class="flex items-end justify-end w-full gap-2 flex-wrap">
                                <Button
                                    variant=ButtonVariant::Icon
                                    on_click=|_| {}
                                    icon=view! { <Youtube /> }.into_any()
                                />
                            </div>
                        </div>
                    </Card>

                    <Card class="w-md">
                        <div class="flex flex-col gap-4 justify-between h-full">
                            <div class="flex flex-row items-center gap-4">
                                <Avatar
                                    url="https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4"
                                    size=52
                                />
                                <div>
                                    <h2 class="text-heading-3 mt-2 mb-2">"Nombre canal"</h2>
                                    <h3 class="text-paragraph-3 mt-2 mb-2">"Subtítulo"</h3>
                                </div>
                            </div>
                            <div>
                                <p class="text-paragraph-3 mb-4">
                                    "Breve descripción del canal o recurso contando qué muestra o para qué sirve."
                                </p>
                            </div>
                            <div class="flex items-end justify-end w-full gap-2 flex-wrap">
                                <Button
                                    variant=ButtonVariant::Icon
                                    on_click=|_| {}
                                    icon=view! { <Youtube /> }.into_any()
                                />
                            </div>
                        </div>
                    </Card>

                    <Card class="w-md">
                        <div class="flex flex-col gap-4 justify-between h-full">
                            <div class="flex flex-row items-center gap-4">
                                <Avatar
                                    url="https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4"
                                    size=52
                                />
                                <div>
                                    <h2 class="text-heading-3 mt-2 mb-2">"Nombre canal"</h2>
                                    <h3 class="text-paragraph-3 mt-2 mb-2">"Subtítulo"</h3>
                                </div>
                            </div>
                            <div>
                                <p class="text-paragraph-3 mb-4">
                                    "Breve descripción del canal o recurso contando qué muestra o para qué sirve."
                                </p>
                            </div>
                            <div class="flex items-end justify-end w-full gap-2 flex-wrap">
                                <Button
                                    variant=ButtonVariant::Icon
                                    on_click=|_| {}
                                    icon=view! { <Youtube /> }.into_any()
                                />
                            </div>
                        </div>
                    </Card>

                    <Card class="w-md">
                        <div class="flex flex-col gap-4 justify-between h-full">
                            <div class="flex flex-row items-center gap-4">
                                <Avatar
                                    url="https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4"
                                    size=52
                                />
                                <div>
                                    <h2 class="text-heading-3 mt-2 mb-2">"Nombre canal"</h2>
                                    <h3 class="text-paragraph-3 mt-2 mb-2">"Subtítulo"</h3>
                                </div>
                            </div>
                            <div>
                                <p class="text-paragraph-3 mb-4">
                                    "Breve descripción del canal o recurso contando qué muestra o para qué sirve."
                                </p>
                            </div>
                            <div class="flex items-end justify-end w-full gap-2 flex-wrap">
                                <Button
                                    variant=ButtonVariant::Icon
                                    on_click=|_| {}
                                    icon=view! { <Youtube /> }.into_any()
                                />
                            </div>
                        </div>
                    </Card>
                </div>
            </div>
        </section>
    }
}
