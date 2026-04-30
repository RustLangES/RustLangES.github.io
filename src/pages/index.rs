use leptos::prelude::*;
use rustlanges_components::button::{Button, Variant as ButtonVariant};

use crate::components::{
    announcement_banner::AnnouncementBanner,
    became_sponsor::BecameSponsorSection,
    community_project::CommunityProjectSection,
    footer::Footer,
    our_community::OurCommunitySection,
    our_sponsors::OurSponsorsSection,
};

#[component]
pub fn Index() -> impl IntoView {
    view! {
        // Hero
        <div class="w-full min-h-[65dvh] rustlang-es-background dark:bg-[#F04906] text-akira flex items-center justify-center py-16">
            <div class="w-full container desktop:max-w-desktop flex flex-col md:flex-row items-center justify-center m-auto gap-8 lg:gap-16 px-6">
                <div class="flex flex-col justify-center gap-8 xl:min-w-lg mx-auto lg:mx-0 text-center lg:text-left">
                    <div class="flex flex-col gap-2">
                        <p class="uppercase font-body text-sm tracking-widest">
                            "Comunidad en español"
                        </p>
                        <h1 class="uppercase leading-tight">
                            "Hacemos"
                            <br />
                            <span class="text-primary-500 dark:text-orange-300">"Posible"</span>
                            <br />
                            "Aprender Rust"
                        </h1>
                    </div>
                    <div class="flex gap-4 flex-wrap font-body justify-center lg:justify-start">
                        <Button
                            variant=ButtonVariant::Secondary
                            on_click=|_| {}
                            label="Aprender"
                        />
                        <Button
                            variant=ButtonVariant::Primary
                            on_click=|_| {}
                            label="Súmate"
                        />
                    </div>
                </div>
                <div class="flex-shrink-0">
                    <img
                        src="/assets/new/logos/ferris-hero.png"
                        alt="Ferris la mascota de Rust"
                        class="w-64 lg:w-80 xl:w-[420px]"
                    />
                </div>
            </div>
        </div>

        <AnnouncementBanner />
        <OurCommunitySection />
        <CommunityProjectSection />
        <OurSponsorsSection />
        <BecameSponsorSection />
        <Footer />
    }
}
