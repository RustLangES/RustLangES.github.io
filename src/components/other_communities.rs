use leptos::*;

use crate::components::icons::discord_icon::DiscordIcon;
use crate::components::icons::telegram_icon::TelegramIcon;

#[component]
pub fn OtherCommunities() -> impl IntoView {
    view! { 
        <section class="bg-orange-100 py-20 drop-shadow-md">
            <div class="container mx-auto px-4">
                <h2 class="text-2xl text-left mb-4">
                    <span class="font-work-sans font-light">"Otras "</span>
                    <span class="font-alfa-slab text-orange-500">"Comunidades"</span>
                    <span class="font-work-sans font-light">" recomendadas "</span>
                </h2>
                <div class="w-full grid grid-cols-5 gap-x-8">
                    <a
                        href="https://discord.gg/DeZKCCyMcq"
                        target="_blank"
                        class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                    >
                        <div>
                            <img src="./RustBCN.webp" width="60" class="rounded-full mb-4" alt="Logo de Rust Barcelona" />
                            <h5 class="text-xl">
                                <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                    "Rust "
                                </span>
                                <span class="font-work-sans text-black">
                                    "Barcelona"
                                </span>
                            </h5>
                            <p class="font-work-sans text-black">
                                "Comunidad de Rust en Barcelona"
                            </p>
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
                            <img src="./Whizzles.webp" width="60" class="mb-4" alt="Logo de Whizzles" />
                            <h5 class="text-xl">
                                <span class="font-work-sans text-black">
                                    "Whizzles"
                                </span>
                            </h5>
                            <p class="font-work-sans text-black">
                                "Comunidad de Open Source con fuertes raices en Rust."
                            </p>
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
                            <img src="./RustMX.png" width="60" class="mb-4" alt="Logo de RustMX" />
                            <h5 class="text-xl">
                                <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                    "Rust"
                                </span>
                                <span class="font-work-sans text-black">
                                    "MX"
                                </span>
                            </h5>
                            <p class="font-work-sans text-black">
                                "Comunidad de Rust en Mexico"
                            </p>
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
                            <img src="./aprender-rust.jpg" width="60" class="rounded-full mb-4" alt="Logo de Aprende Rust" />
                            <h5 class="text-xl">
                                <span class="font-work-sans text-black">
                                    "Aprende "
                                </span>
                                <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                    "Rust"
                                </span>
                            </h5>
                            <p class="font-work-sans text-black">
                                "Comunidad de gente queriendo aprender."
                            </p>
                        </div>
                        <span class="ml-auto">
                            <TelegramIcon size=30 />
                        </span>
                    </a>
                    <a
                        href="https://t.me/rust_lang_es"
                        target="_blank"
                        class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                    >
                        <div>
                            <img src="./RustGeneral.jpg" width="60" class="rounded-full mb-4" alt="Logo de Rust Español" />
                            <h5 class="text-xl">
                                <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                    "Rust"
                                </span>
                                <span class="font-work-sans text-black">
                                    " Español"
                                </span>
                            </h5>
                            <p class="font-work-sans text-black">
                                "Anteriormente Rust Argentina, hoy una comunidad internacional."
                            </p>
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
