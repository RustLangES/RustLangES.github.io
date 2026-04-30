use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant as ButtonVariant},
    card::Card,
    icons::Youtube,
};

#[derive(Clone)]
struct ChannelData {
    name: &'static str,
    subtitle: &'static str,
    description: &'static str,
    avatar_url: &'static str,
    youtube_url: &'static str,
}

#[component]
pub fn Channels() -> impl IntoView {
    let channels: Vec<ChannelData> = vec![
        ChannelData {
            name: "Nombre canal",
            subtitle: "Subtítulo",
            description: "Breve descripción del canal o recurso contando qué muestra o para qué sirve.",
            avatar_url: "https://avatars.githubusercontent.com/u/136520331?v=4",
            youtube_url: "#",
        },
        ChannelData {
            name: "Nombre canal",
            subtitle: "Subtítulo",
            description: "Breve descripción del canal o recurso contando qué muestra o para qué sirve.",
            avatar_url: "https://avatars.githubusercontent.com/u/70247585?v=4",
            youtube_url: "#",
        },
        ChannelData {
            name: "Nombre canal",
            subtitle: "Subtítulo",
            description: "Breve descripción del canal o recurso contando qué muestra o para qué sirve.",
            avatar_url: "https://avatars.githubusercontent.com/u/56278796?v=4",
            youtube_url: "#",
        },
    ];

    view! {
        <section class="bg-white dark:bg-neutral-900 py-16 px-4 w-full">
            <div class="container max-w-7xl mx-auto">
                <h2 class="text-h2 text-center mb-12">"Canales recomendados"</h2>
                <div class="flex flex-row justify-center items-stretch gap-6 flex-wrap md:flex-nowrap">
                    {channels.into_iter().map(|ch| view! {
                        <Card class="w-full max-w-sm p-6">
                            <div class="flex flex-col gap-4 justify-between h-full">
                                <div class="flex flex-row items-center gap-4">
                                    <Avatar url=ch.avatar_url size=52 />
                                    <div>
                                        <h3 class="font-bold text-base">{ch.name}</h3>
                                        <p class="text-sm text-neutral-500 dark:text-neutral-400">{ch.subtitle}</p>
                                    </div>
                                </div>
                                <p class="text-sm text-neutral-600 dark:text-neutral-400 flex-1 leading-relaxed">
                                    {ch.description}
                                </p>
                                <div class="flex items-end justify-end">
                                    <a href=ch.youtube_url target="_blank">
                                        <Button
                                            variant=ButtonVariant::Icon
                                            on_click=|_| {}
                                            icon=view! { <Youtube /> }.into_any()
                                        />
                                    </a>
                                </div>
                            </div>
                        </Card>
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
