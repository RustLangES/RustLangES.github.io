use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Variant as ButtonVariant, button_class},
    card::Card,
    chip::{Chip, Variant as ChipVariant},
};

use crate::models::Community;

#[component]
pub fn CommunityCard(community: &'static Community) -> impl IntoView {
    let button_link = community.link.as_ref().map(|(link, link_icon)| {
        view! {
            <a
                class=button_class(
                    ButtonVariant::Icon,
                    Some("-top-3 -right-2 absolute".to_string()),
                )
                href=*link
            >
                {link_icon.to_view()}
            </a>
        }
    });

    view! {
        <div class="min-w-87.5 max-w-md h-full mt-5">
            <Card class="min-w-87.5 max-w-md h-full p-4">
                <div class="flex flex-col gap-4 justify-between h-full">
                    <div class="flex flex-col items-center gap-4  h-full justify-stretch p-2">
                        <div class="relative">
                            {button_link} <Avatar url=community.logo size=100 />
                        </div>

                        <h2 class="text-h3 mt-2">{community.name}</h2>
                        <Chip label=community.country variant=ChipVariant::Location />
                        <p class="text-paragraph-3 mb-4 text-center text-balance mx-auto">
                            {community.description}
                        </p>
                    </div>
                </div>
            </Card>
        </div>
    }
}
