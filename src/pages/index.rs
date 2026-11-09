use leptos::prelude::*;
use rustlanges_components::button::{Button, Variant as ButtonVariant};

use crate::components::{
    became_sponsor::BecameSponsorSection, community_project::CommunityProjectSection,
    footer::Footer, our_community::OurCommunitySection, our_sponsors::OurSponsorsSection,
};

#[allow(unused_variables)]
#[component]
pub fn Index() -> impl IntoView {
    let (gcount, wcount) = signal(20);
    //  xl:max-w-[110rem]
    view! {
        <div class="w-full md:min-h-[65dvh] py-12 md:py-0 rustlang-es-background dark:bg-[#F04906] text-akira flex items-center justify-center">
            <div class="w-full container desktop:max-w-desktop px-4 flex flex-col md:flex-row items-center justify-center m-auto gap-8 lg:gap-0">
                <div class="flex flex-col justify-center gap-8 w-full md:w-auto xl:min-w-lg mx-auto lg:mx-0 text-center lg:text-left">
                    <div class="flex flex-col gap-2">
                        <p class="uppercase font-body text-caption md:text-base">
                            Comunidad en español
                        </p>
                        <p class="uppercase heading text-[22px] tablet:text-[28px] desktop:text-[40px]">
                            "Hacemos "
                            <span class="text-primary-500 dark:text-orange-300">"Posible"</span> " "
                            <br class="md:hidden" /> "Aprender Rust"
                        </p>
                    </div>
                    <div class="grid grid-cols-2 md:flex gap-4 md:flex-wrap max-w-full text-[14px] font-body justify-center lg:justify-start">
                        <Button
                            variant=ButtonVariant::Secondary
                            class="w-full md:w-fit"
                            on_click=|_| {}
                            label="Aprender"
                        />
                        <Button
                            variant=ButtonVariant::Primary
                            class="w-full md:w-fit"
                            on_click=|_| {}
                            label="Súmate"
                        />
                    </div>
                </div>
                <div class="w-full max-w-sm md:w-auto md:max-w-none">
                    <img
                        src="/assets/new/logos/ferris-hero.png"
                        alt=""
                        class="w-full md:w-auto md:min-w-xs lg:min-w-sm xl:min-w-lg"
                    />
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
