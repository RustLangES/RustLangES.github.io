use leptos::*;

use crate::components::icons::discord_icon::DiscordIcon;
use crate::components::icons::telegram_icon::TelegramIcon;

#[component]
pub fn OtherCommunities(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="bg-orange-100 py-20">
            <div class="container mx-auto px-4">
                <div class="w-full grid grid-cols-4 gap-x-8">
                    <a
                        href="https://discord.gg/DeZKCCyMcq"
                        target="_blank"
                        class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                    >
                        <div>
                            <img src="./RustBCN.webp" width="60" class="rounded-full mb-4" />
                            <h5 class="text-xl">
                                <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                    "Rust "
                                </span>
                                <span class="font-work-sans text-black">
                                    "Barcelona"
                                </span>
                            </h5>
                        </div>
                        <span class="ml-auto">
                            <DiscordIcon size=30 />
                        </span>
                    </a>
                    <a
                        href="https://discord.gg/b9EbVnSkuw"
                        target="_blank"
                        class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                    >
                        <div>
                            <img src="./Whizzles.webp" width="60" class="rounded-full mb-4" />
                            <h5 class="text-xl">
                                <span class="font-work-sans text-black">
                                    "Whizzles"
                                </span>
                            </h5>
                        </div>
                        <span class="ml-auto">
                            <DiscordIcon size=30 />
                        </span>
                    </a>
                    <a
                        href="https://t.me/RustMX"
                        target="_blank"
                        class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                    >
                        <div>
                            <img src="./RustMX.png" width="60" class="rounded-full mb-4" />
                            <h5 class="text-xl">
                                <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                    "Rust"
                                </span>
                                <span class="font-work-sans text-black">
                                    "MX"
                                </span>
                            </h5>
                        </div>
                        <span class="ml-auto">
                            <TelegramIcon size=30 />
                        </span>
                    </a>
                    <a
                        href="https://t.me/aprenderrust"
                        target="_blank"
                        class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                    >
                        <div>
                            <img src="./RustGeneral.jpg" width="60" class="rounded-full mb-4" />
                            <h5 class="text-xl">
                                <span class="font-work-sans text-black">
                                    "Aprende "
                                </span>
                                <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                    "Rust"
                                </span>
                            </h5>
                        </div>
                        <span class="ml-auto">
                            <TelegramIcon size=30 />
                        </span>
                    </a>
                </div>
            </div>
        </section>
    }
}
