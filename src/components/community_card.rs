use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Variant as ButtonVariant, button_class},
    card::Card,
    chip::{Chip, Variant as ChipVariant},
    icons::{Discord, Link, Telegram},
};

use crate::models::CommunityItem;

#[component]
pub fn CommunityCard(community: &'static CommunityItem) -> impl IntoView {
    let icon = match community.icon {
        "telegram" => view! { <Telegram /> }.into_any(),
        "discord" => view! { <Discord /> }.into_any(),
        _ => view! { <Link /> }.into_any(),
    };

    view! {
        <div class="min-w-87.5 max-w-md h-full mt-5">
            <Card class="min-w-87.5 max-w-md h-full p-4">
                <div class="flex flex-col gap-4 justify-between h-full">
                    <div class="flex flex-col items-center gap-4  h-full justify-stretch p-2">
                        <div class="relative">
                            <a
                                class=button_class(
                                    ButtonVariant::Icon,
                                    Some("-top-3 -right-2 absolute".to_string()),
                                )
                                href=community.link
                            >
                                {icon}
                            </a>
                            <Avatar url=community.brand_src alt=community.brand_alt size=100 />
                        </div>
                        <h2 class="text-h3 mt-2">{community.name.concat()}</h2>
                        {community
                            .country
                            .map(|country| {
                                view! { <Chip label=country variant=ChipVariant::Location /> }
                            })}
                        <p class="text-paragraph-3 mb-4 text-center text-balance mx-auto">
                            {community.description}
                        </p>
                    </div>
                </div>
            </Card>
        </div>
    }
}
