use leptos::prelude::*;
use leptos_router::hooks::use_url;
use rustlanges_components::{
    badge::{Type, Variant},
    button::{Button, Variant as ButtonVariant},
};

use crate::components::{
    became_sponsor::BecameSponsorSection, community_project::CommunityProjectSection,
    footer::Footer, our_community::OurCommunitySection, our_sponsors::OurSponsorsSection,
};

#[component]
pub fn Index() -> impl IntoView {
    let (gcount, wcount) = signal(20);
    //  xl:max-w-[110rem]
    view! {
        <div class="w-full h-[65dvh] rustlang-es-background dark:bg-[#F04906] text-akira flex items-center justify-center">
            <div class="w-full container  flex flex-row items-center justify-center m-auto">
                <div class="flex flex-col justify-center gap-8">
                    <div class="flex flex-col gap-2">
                        <p class="uppercase">Comunidad en español</p>
                        <p class="uppercase text-h1">
                            Hacemos <span class="text-primary-500">Posible</span>Aprender Rust
                        </p>
                    </div>
                    <div class="flex gap-4 flex-wrap max-w-full">
                        <Button variant=ButtonVariant::Secondary on_click=|_| {} label="Aprender" />
                        <Button variant=ButtonVariant::Primary on_click=|_| {} label="Súmate" />
                    </div>
                </div>
                <div>
                    <img src="/assets/new/logos/ferris-hero.png" alt="" />
                </div>
            </div>
        </div>
        <OurCommunitySection />
        <CommunityProjectSection />
        <OurSponsorsSection />
        <BecameSponsorSection />
        <Footer />
    }
}
