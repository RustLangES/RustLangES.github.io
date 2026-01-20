use leptos::{component, view, IntoView};

use crate::components::{
    button_link::ButtonLink,
    icons::{DiscordIcon, GithubIcon, LinkedinIcon, TelegramIcon},
};

#[component]
pub fn OurCommunities() -> impl IntoView {
    view! {
        <section class=" bg-orange-300/30 dark:bg-transparent">
            <div class="container mx-auto px-4">
                <div class="flex flex-col items-center py-20 gap-y-6">
                    <h2 class="text-4xl text-center mb-4">
                        <span class="font-work-sans font-light">"Únete a nuestra "</span>
                        <span class="font-alfa-slab text-orange-500">"Comunidad"</span>
                    </h2>
                    <div class="flex items-center gap-x-12 gap-y-6 flex-col *:w-full sm:flex-row">
                        <ButtonLink
                            href="https://discord.gg/4ng5HgmaMg"
                            shadow="box"
                            color="white"
                            size="big"
                        >
                            <DiscordIcon size=30 />
                            "Discord"
                        </ButtonLink>
                        <ButtonLink
                            href="https://github.com/RustLangES"
                            shadow="box"
                            color="white"
                            size="big"
                        >
                            <GithubIcon size=30 />
                            "Github"
                        </ButtonLink>
                        <ButtonLink
                            href="https://www.linkedin.com/company/rustlanges"
                            color="white"
                            size="big"
                            shadow="box"
                        >
                            <LinkedinIcon size=30 />
                            "Linkedin"
                        </ButtonLink>
                        <ButtonLink
                            href="https://t.me/rust_es"
                            shadow="box"
                            color="white"
                            size="big"
                        >
                            <TelegramIcon size=30 />
                            "Telegram"
                        </ButtonLink>
                    </div>
                </div>
            </div>
        </section>
    }
}
