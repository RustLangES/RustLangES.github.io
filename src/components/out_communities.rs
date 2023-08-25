use leptos::*;

use crate::components::atom::discord_icon::DiscordIcon;
use crate::components::atom::github_icon::GithubIcon;
use crate::components::atom::telegram_icon::TelegramIcon;

#[component]
pub fn OurCommunities(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section class="bg-orange-300/30">
            <div class="container mx-auto px-4">
                <div class="flex flex-col items-center py-20 gap-y-6">
                    <h2 class="text-4xl text-center mb-4">
                        <span class="font-work-sans font-light">"Unete a nuestra "</span>
                        <span class="font-alfa-slab text-orange-500">"Comunidad"</span>
                    </h2>
                    <div class="flex items-center gap-x-12">
                        <a
                            href="https://discord.gg/4ng5HgmaMg"
                            target="_blank"
                            class="tracking-widest font-light border border-black flex gap-x-4 items-center px-4 h-12 bg-orange-100 drop-shadow-[4px_4px_0_rgba(0,0,0)] hover:drop-shadow-[0_0_0_rgba(0,0,0)] transition"
                        >
                            <DiscordIcon />
                            <span>"Discord"</span>
                        </a>
                        <a
                            href="https://github.com/RustLangES"
                            target="_blank"
                            class="tracking-widest font-light border border-black flex gap-x-4 items-center px-4 h-12 bg-orange-100 drop-shadow-[4px_4px_0_rgba(0,0,0)] hover:drop-shadow-[0_0_0_rgba(0,0,0)] transition"
                        >
                            <GithubIcon />
                            <span>"Discord"</span>
                        </a>
                        <a
                            href="https://t.me/rust_es"
                            target="_blank"
                            class="tracking-widest font-light border border-black flex gap-x-4 items-center px-4 h-12 bg-orange-100 drop-shadow-[4px_4px_0_rgba(0,0,0)] hover:drop-shadow-[0_0_0_rgba(0,0,0)] transition"
                        >
                            <TelegramIcon />
                            <span>"Discord"</span>
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}