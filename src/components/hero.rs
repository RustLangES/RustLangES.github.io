use leptos::*;

use crate::components::atom::discord_icon::DiscordIcon;
use crate::components::atom::github_icon::GithubIcon;
use crate::components::atom::telegram_icon::TelegramIcon;

#[component]
pub fn Hero(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section class="w-full h-screen">
            <div class="h-full flex flex-col">
                <div class="border-b border-b-black/20">
                    <header class="container mx-auto px-4 flex items-center justify-between">
                        <div class="flex items-center gap-x-4">
                            <img
                                src="https://www.rust-lang.org/static/images/rust-logo-blk.svg"
                                class="max-h-20 rounded-full"
                                alt="Rust Lang en Español"
                            />
                            <h1 class="text-xl font-alfa-slab">Rust Lang en Español</h1>
                        </div>
                        <nav class="flex items-center gap-5">
                            <a
                                class="tracking-widest font-light border rounded-md border-black flex items-center px-4 h-9 hover:bg-black/10"
                                href="https://github.com/RustLangES"
                                target="_blank"
                            >
                                Github
                            </a>
                            <a
                                class="tracking-widest font-light border rounded-md border-black flex items-center px-4 h-9 hover:bg-black/10"
                                href="https://discord.gg/4ng5HgmaMg"
                                target="_blank"
                            >
                                Discord
                            </a>
                        </nav>
                    </header>
                </div>
                <div class="flex-1 flex items-center justify-center">
                    <div class="text-center">
                        <img src="./rhq3ezvso9611.png" width="400" class="mx-auto" />
                        <h2 class="p-6 text-4xl font-bold">"Bienvenidos a Rust Lang en Español"</h2>
                        <p class="px-10 pb-10 text-2xl">"Una comunidad de gente mal intencionada y tonta."</p>
                    </div>
                </div>
                <div class="border-t border-t-black/20 py-6">
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
                </div>
            </div>
            <img
                src="./background.jpg"
                class="absolute inset-0 w-full h-full object-cover -z-[1] opacity-5 mix-blend-luminosity"
            />
        </section>
    }
}
