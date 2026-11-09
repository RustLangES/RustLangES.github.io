use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant as ButtonVariant},
    card::Card,
    icons::Youtube,
};

use crate::components::carousel::Carousel;

#[allow(non_snake_case)]
pub fn Channels() -> impl IntoView {
    view! {
        <section class="bg-light dark:bg-dark py-12 lg:p-20 w-full">
            <div class="container px-4 xl:max-w-7xl lg:px-36 mx-auto">
                <h2 class="text-heading-2 md:text-heading-1 font-bold text-center mb-4 md:mb-8">
                    "Canales Recomendados"
                </h2>
            </div>

            <Carousel
                count=5
                class="md:flex-wrap md:justify-center md:overflow-visible".to_string()
            >
                {(0..5)
                    .map(|_| {
                        view! {
                            <Card class="shrink-0 snap-start w-[85%] max-w-md md:w-md flex flex-col gap-4 justify-between">
                                <div class="flex flex-row items-center gap-4">
                                    <Avatar
                                        url="https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4"
                                        size=52
                                    />
                                    <div>
                                        <h3 class="text-h5 font-bold">"Nombre canal"</h3>
                                        <p class="text-paragraph-1">"Subtítulo"</p>
                                    </div>
                                </div>
                                
                                <p class="text-paragraph-1">
                                    "Breve descripción del canal o recurso contando qué muestra o para qué sirve."
                                </p>

                                <div class="flex items-end justify-end w-full gap-2 flex-wrap">
                                    <Button
                                        variant=ButtonVariant::Icon
                                        on_click=|_| {}
                                        icon=view! { <Youtube /> }.into_any()
                                    />
                                </div>
                            </Card>
                        }
                    })
                    .collect_view()}
            </Carousel>
        </section>
    }
}
