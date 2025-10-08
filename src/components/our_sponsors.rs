use leptos::{prelude::*, *};

use crate::components::sponsor_block::SponsorBlock;

#[component]
pub fn OurSponsorsSection() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center w-full gap-8 py-20 dark:bg-neutral-900">
            <div class="container flex flex-col justify-center items-center gap-8">
                <h2 class="text-h2 mb-4">"Nuestros sponsors"</h2>
                <p class="text-center mb-8 max-w-lg">
                    Todos nuestros eventos y actividades son
                    <span class="font-bold">gratuitas</span>
                    gracias a las organizaciones que apoyan nuestro trabajo.
                </p>

                <div>
                    <img src="/assets/new/logos/ferris-hero.png" alt="" width="300" />
                </div>

                <div class="flex justify-center items-center relative z-[2] -mt-11">
                    <SponsorBlock class="text-primary-500" />
                    <SponsorBlock class="text-black">
                        <img
                            src="/assets/sponsors/sysarmy.webp"
                            alt="SysArmy"
                            class="max-h-[50px]"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-secondary-500" />
                    <SponsorBlock class="text-white">
                        <img
                            src="/assets/new/sponsors/testing-bolivia.png"
                            alt="Testing Bolivia"
                            class="max-h-[50px] mix-blend-difference"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-500" />
                </div>
                <div class="flex justify-center items-center relative z-[1] -mt-11">
                    <SponsorBlock class="text-[#00c39d]">
                        <img
                            src="/assets/new/sponsors/frontendcafe.png"
                            alt="FrontendCafe"
                            class="max-h-[50px] shadow-frontendcafe"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-200" />
                    <SponsorBlock class="text-[#193270]">
                        <img
                            src="/assets/new/sponsors/universidad_nur.png"
                            alt="Universidad Nur"
                            class="max-h-[50px]"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-200" />
                    <SponsorBlock class="text-black">
                        <img
                            src="/assets/sponsors/shuttle.webp"
                            alt="ShuttleRS"
                            class="max-h-[50px] mix-blend-difference"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-200" />
                </div>

            // <SponsorBlock class="text-[#00c39d]">
            // <img src="/assets/sponsors/crabnebula.png" alt="Crabnebula" class="max-h-[50px]" />
            // </SponsorBlock>
            </div>
        </section>
    }
}
