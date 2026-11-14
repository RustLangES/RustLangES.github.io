use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Variant as ButtonVariant, button_class},
    card::Card,
    icons::{Link, Youtube},
};

use crate::{components::carousel::Carousel, extras::CHANNELS, models::ChannelItem};

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
                count=CHANNELS.len()
                class="md:flex-wrap md:justify-center md:overflow-visible".to_string()
            >
                {CHANNELS
                    .iter()
                    .map(|channel| {
                        let ChannelItem { name, description, avatar, youtube, twitch } = channel;
                        view! {
                            <Card class="shrink-0 snap-start w-[85%] max-w-md md:w-md flex flex-col gap-4 justify-between">
                                <div class="flex flex-row items-center gap-4">
                                    <Avatar url=*avatar alt=*name size=52 />
                                    <h3 class="text-h5 font-bold">{*name}</h3>
                                </div>

                                <p class="text-paragraph-1">{*description}</p>

                                <div class="flex items-end justify-end w-full gap-2 flex-wrap">
                                    {youtube
                                        .map(|href| {
                                            view! {
                                                <a
                                                    class=button_class(ButtonVariant::Icon, None)
                                                    href=href
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    aria-label="YouTube"
                                                >
                                                    <Youtube />
                                                </a>
                                            }
                                        })}
                                    {twitch
                                        .map(|href| {
                                            view! {
                                                <a
                                                    class=button_class(ButtonVariant::Icon, None)
                                                    href=href
                                                    target="_blank"
                                                    rel="noopener noreferrer"
                                                    aria-label="Twitch"
                                                >
                                                    <Link />
                                                </a>
                                            }
                                        })}
                                </div>
                            </Card>
                        }
                    })
                    .collect_view()}
            </Carousel>
        </section>
    }
}
