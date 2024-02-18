use leptos::{IntoView, component, tracing, view};

use crate::components::{
    button_link::ButtonLink,
    icons::{DiscordIcon, GithubIcon, TelegramIcon},
};

#[component]
pub fn OurCommunities() -> impl IntoView {
    view! {
        <section class=" bg-orange-300/30">
            <div class="container mx-auto px-4">
                <div class="flex flex-col items-center py-20 gap-y-6">
                    <h2 class="text-4xl text-center mb-4">
                        <span class="font-work-sans font-light">"Unete a nuestra "</span>
                        <span class="font-alfa-slab text-orange-500">"Comunidad"</span>
                    </h2>
                    <div class="flex items-center gap-x-12 gap-y-6 flex-col sm:flex-row">
                        <ButtonLink href="https://discord.gg/4ng5HgmaMg" color="white" size="big">
                            <DiscordIcon size=30/>
                            "Discord"
                        </ButtonLink>
                        <ButtonLink href="https://github.com/RustLangES" color="white" size="big">
                            <GithubIcon size=30/>
                            "Github"
                        </ButtonLink>
                        <ButtonLink href="https://t.me/rust_es" color="white" size="big">
                            <TelegramIcon size=30/>
                            "Telegram"
                        </ButtonLink>
                    </div>
                </div>
            </div>
        </section>
    }
}