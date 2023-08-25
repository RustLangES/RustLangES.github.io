use leptos::*;

use crate::components::atom::discord_icon::DiscordIcon;
use crate::components::atom::github_icon::GithubIcon;
use crate::components::atom::telegram_icon::TelegramIcon;

#[component]
pub fn Hero(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section class="w-full flex flex-col">
            <header class="border-b border-b-black/20">
                <div class="container mx-auto px-4 flex items-center justify-between">
                    <div class="flex items-center gap-x-4">
                        <img
                            src="https://www.rust-lang.org/static/images/rust-logo-blk.svg"
                            class="max-h-20 rounded-full"
                            alt="Rust Lang en Español"
                        />
                    </div>
                    <nav>
                        <ul class="flex items-center gap-5">
                            <li>
                                <a href="#">Aprende</a>
                            </li>
                            <li>
                                <a href="#">Comunidad</a>
                            </li>
                            <li>
                                <a href="#">Blog</a>
                            </li>
                            <li class="ml-4">
                                <a
                                    class="tracking-widest font-light border border-black flex items-center px-4 h-9 bg-orange-300 hover:bg-black hover:text-white drop-shadow-[4px_4px_0_rgba(0,0,0)] hover:drop-shadow-[0_0_0_rgba(0,0,0)] transition"
                                    href="https://github.com/RustLangES"
                                    target="_blank"
                                >
                                    Github
                                </a>
                            </li>
                            <li>
                                <a
                                    class="tracking-widest font-light border border-black flex items-center px-4 h-9 bg-orange-300 hover:bg-black hover:text-white drop-shadow-[4px_4px_0_rgba(0,0,0)] hover:drop-shadow-[0_0_0_rgba(0,0,0)] transition"
                                    href="https://discord.gg/4ng5HgmaMg"
                                    target="_blank"
                                >
                                    Discord
                                </a>
                            </li>
                        </ul>
                    </nav>
                </div>
            </header>
            <div class="flex items-center justify-center py-32">
                <div class="flex items-center gap-x-20">
                    <figure>
                        <img src="./rhq3ezvso9611.png" width="500" class="mx-auto" />
                    </figure>
                    <div class="">
                        <h1 class="flex flex-col mb-4 gap-y-2">
                            <span class="font-work-sans text-4xl font-light">"Bienvenidos a"</span>
                            <span class="font-alfa-slab text-orange-500 text-8xl">"Rust Lang"</span>
                            <span class="font-work-sans text-5xl font-semibold">"En Español"</span>
                        </h1>
                        <p class="font-work-sans font-light">
                            "Una comunidad de gente mal intencionada y tonta."
                        </p>
                    </div>
                </div>
            </div>
            /* <div class="py-6">
                <div class="container mx-auto">
                    <h2 class="text-lg text-center mb-2">Comunidad</h2>
                    <p class="mb-4 text-center text-xs font-light">
                        Unete a nuestra comunidad en Discord, Telegram y Github
                    </p>
                    <div class="flex justify-center gap-x-12">
                        <a href="https://discord.gg/4ng5HgmaMg" target="_blank">
                            <DiscordIcon />
                        </a>
                        <a href="https://github.com/RustLangES" target="_blank">
                            <GithubIcon />
                        </a>
                        <a href="https://t.me/rust_es" target="_blank">
                            <TelegramIcon />
                        </a>
                    </div>
                </div>
            </div> */
        </section>
    }
}
