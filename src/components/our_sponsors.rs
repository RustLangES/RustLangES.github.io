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

                <div class="flex justify-center items-center relative z-[5] -mt-[42px]">
                    <SponsorBlock class="text-primary-500 max-w-20 md:max-w-28 lg:max-w-max" />
                    <SponsorBlock class="text-black max-w-20 md:max-w-28 lg:max-w-max">
                        <img
                            src="/assets/sponsors/sysarmy.webp"
                            alt="SysArmy"
                            class="max-h-[25px] lg:max-h-[50px] max-w-20 md:max-w-28 lg:max-w-max"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-secondary-500 max-w-20 md:max-w-28 lg:max-w-max" />
                    <SponsorBlock class="text-white max-w-20 md:max-w-28 lg:max-w-max  hidden md:block">
                        <img
                            src="/assets/new/sponsors/testing-bolivia.png"
                            alt="Testing Bolivia"
                            class="max-h-[50px] mix-blend-difference max-w-20 md:max-w-28 lg:max-w-max  hidden md:block"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-500  max-w-20 md:max-w-28 lg:max-w-max  hidden md:block" />
                </div>
                <div class="flex justify-center items-center relative z-[4] -mt-[70px] md:-mt-[56px] lg:-mt-[42px]">
                    <SponsorBlock class="text-white max-w-20 md:max-w-28 lg:max-w-max md:hidden">
                        <img
                            src="/assets/new/sponsors/testing-bolivia.png"
                            alt="Testing Bolivia"
                            class="max-h-[25px] md:max-h-[50px] mix-blend-difference max-w-20 md:max-w-28 lg:max-w-max md:hidden"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-200  max-w-20 md:max-w-28 lg:max-w-max md:hidden" />

                    <SponsorBlock class="text-[#00c39d] max-w-20 md:max-w-28 lg:max-w-max">
                        <img
                            src="/assets/new/sponsors/frontendcafe.png"
                            alt="FrontendCafe"
                            class="max-h-[25px] lg:max-h-[50px] shadow-frontendcafe max-w-20 md:max-w-28 lg:max-w-max"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-200 max-w-20 md:max-w-28 lg:max-w-max" />
                    <SponsorBlock class="text-[#193270] max-w-20 md:max-w-28 lg:max-w-max hidden md:block">
                        <img
                            src="/assets/new/sponsors/universidad_nur.png"
                            alt="Universidad Nur"
                            class="max-h-[50px] max-w-20 md:max-w-28 lg:max-w-max hidden md:block"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-200 max-w-20 md:max-w-28 lg:max-w-max hidden md:block" />
                    <SponsorBlock class="text-black max-w-20 md:max-w-28 lg:max-w-max hidden md:block">
                        <img
                            src="/assets/sponsors/shuttle.webp"
                            alt="ShuttleRS"
                            class="max-h-[50px] mix-blend-difference max-w-20 md:max-w-28 lg:max-w-max hidden md:block"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-200 max-w-20 md:max-w-28 lg:max-w-max hidden md:block" />
                </div>
                <div class="flex justify-center items-center relative z-[3] -mt-[70px]  md:-mt-[56px] lg:-mt-[42px]">
                    <SponsorBlock class="text-[#193270] max-w-20 md:max-w-28 lg:max-w-max md:hidden">
                        <img
                            src="/assets/new/sponsors/universidad_nur.png"
                            alt="Universidad Nur"
                            class="max-h-[25px] md:max-h-[50px] max-w-20 md:max-w-28 lg:max-w-max md:hidden"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-secondary-300 max-w-20 md:max-w-28 lg:max-w-max md:hidden" />
                    <SponsorBlock class="text-black max-w-20 md:max-w-28 lg:max-w-max md:hidden">
                        <img
                            src="/assets/sponsors/shuttle.webp"
                            alt="ShuttleRS"
                            class="max-h-[30px] md:max-h-[50px] mix-blend-difference max-w-20 md:max-w-28 lg:max-w-max md:hidden"
                        />
                    </SponsorBlock>

                    // End mobile

                    <SponsorBlock class="text-primary-500 max-w-20 md:max-w-28 lg:max-w-max hidden md:block">
                        <img
                            src="/assets/new/sponsors/cloudflare.svg"
                            alt="Cloudflare"
                            class="max-h-[40px] max-w-20 md:max-w-28 lg:max-w-max hidden md:block"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-transparent stroke-0 max-w-20 md:max-w-28 lg:max-w-max hidden md:block" />
                    <SponsorBlock class="text-[#051024] max-w-20 md:max-w-28 lg:max-w-max hidden md:block">
                        <img
                            src="/assets/new/sponsors/crabnebula.svg"
                            alt="Crabnebula"
                            class="max-h-[40px] mix-blend-difference max-w-20 lg:max-w-32 hidden md:block"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-secondary-300 max-w-20 md:max-w-28 lg:max-w-max hidden md:block" />
                    <SponsorBlock class="text-neutral-950 max-w-20 md:max-w-28 lg:max-w-max hidden md:block">
                        <img
                            src="/assets/new/sponsors/tauri-logo.png"
                            alt="Tauri"
                            class="max-h-[50px] max-w-20 lg:max-w-30 hidden md:block"
                        />
                    </SponsorBlock>
                </div>
                <div class="flex justify-center items-center relative z-[2] -mt-[70px]  md:-mt-[56px] lg:-mt-[42px]">
                    <SponsorBlock class="text-primary-500 max-w-20 md:max-w-28 lg:max-w-max md:hidden">
                        <img
                            src="/assets/new/sponsors/cloudflare.svg"
                            alt="Cloudflare"
                            class="max-h-[25px] mb-2 md:mb-0 md:max-h-[40px] max-w-20 md:max-w-28 lg:max-w-max md:hidden "
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-transparent max-w-20 md:max-w-28 lg:max-w-max md:hidden" />
                    <SponsorBlock class="text-[#051024] max-w-20 md:max-w-28 lg:max-w-max md:hidden">
                        <img
                            src="/assets/new/sponsors/crabnebula.svg"
                            alt="Crabnebula"
                            class="max-h-[25px] md:max-h-[40px] mix-blend-difference max-w-20 lg:max-w-32 md:hidden -mt-3"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-neutral-950 max-w-20 md:max-w-28 lg:max-w-max md:hidden">
                        <img
                            src="/assets/new/sponsors/tauri-logo.png"
                            alt="Tauri"
                            class="max-h-[20px] md:max-h-[50px] max-w-20 lg:max-w-30 md:hidden"
                        />
                    </SponsorBlock>

                    // End mobile

                    <SponsorBlock class="text-secondary-300 max-w-20 md:max-w-28 lg:max-w-max hidden md:block" />
                    <SponsorBlock class="text-primary-200 max-w-20 md:max-w-28 lg:max-w-max hidden md:block">
                        <img
                            src="/assets/new/sponsors/heavy-duty-builders.png"
                            alt="FrontendCafe"
                            class="max-h-[50px] shadow-frontendcafe max-w-20 md:max-w-28 lg:max-w-max hidden md:block"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-white max-w-20 md:max-w-28 lg:max-w-max hidden md:block">
                        <img
                            src="/assets/new/sponsors/aws.svg"
                            alt="Amazon Web Services"
                            class="max-h-[34px] md:max-h-[50px] max-w-20 md:max-w-28 lg:max-w-max hidden md:block"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-200 max-w-20 md:max-w-28 lg:max-w-max hidden md:block" />
                    <SponsorBlock class="text-black max-w-20 md:max-w-28 lg:max-w-max hidden md:block">
                        <img
                            src="/assets/sponsors/shuttle.webp"
                            alt="ShuttleRS"
                            class="max-h-[50px] mix-blend-difference max-w-20 md:max-w-28 lg:max-w-max hidden md:block"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-primary-200 max-w-20 md:max-w-28 lg:max-w-max hidden md:block" />
                </div>
                <div class="flex justify-center items-center relative z-[1] -mt-[70px]  md:-mt-[56px] lg:-mt-[42px]">
                    <SponsorBlock class="text-primary-200 max-w-20 md:max-w-28 lg:max-w-max md:hidden">
                        <img
                            src="/assets/new/sponsors/heavy-duty-builders.png"
                            alt="FrontendCafe"
                            class="max-h-[25px] md:max-h-[50px] shadow-frontendcafe max-w-20 md:max-w-28 lg:max-w-max md:hidden"
                        />
                    </SponsorBlock>
                    <SponsorBlock class="text-secondary-500 max-w-20 md:max-w-28 lg:max-w-max md:hidden" />
                    <SponsorBlock class="text-white max-w-20 md:max-w-28 lg:max-w-max md:hidden">
                        <img
                            src="/assets/new/sponsors/aws.svg"
                            alt="Amazon Web Services"
                            class="max-h-[34px] md:max-h-[50px] max-w-20 md:max-w-28 lg:max-w-max md:hidden"
                        />
                    </SponsorBlock>
                </div>
            </div>
        </section>
    }
}
