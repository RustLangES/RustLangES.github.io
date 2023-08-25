use leptos::*;

use crate::components::atom::discord_icon::DiscordIcon;
use crate::components::atom::github_icon::GithubIcon;
use crate::components::atom::telegram_icon::TelegramIcon;

#[component]
pub fn CommunityLinks(
    cx: Scope
) -> impl IntoView {
    view!{ cx,
        // TODO: Fix the border
        <div class="custom-shape-divider-top-1692767000 fill-shape-fill-light dark:fill-shape-fill-dark ">
            <svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 120" preserveAspectRatio="none">
                <path d="M321.39,56.44c58-10.79,114.16-30.13,172-41.86,82.39-16.72,168.19-17.73,250.45-.39C823.78,31,906.67,72,985.66,92.83c70.05,18.48,146.53,26.09,214.34,3V0H0V27.35A600.21,600.21,0,0,0,321.39,56.44Z" class="shape-fill"></path>
            </svg>
        </div>
        <div class="bg-slate-300 dark:bg-zinc-800 w-full bg-no-repeat bg-center flex justify-center items-center flex-col pt-10 pb-10 drop-shadow-2xl shadow-slate-700 dark:shadow-slate-600 ">
            <h2 class="p-6 text-4xl">"Comunidad"</h2>
            <p class="px-10 pb-10 text-left">"Unete a nuestra comunidad en Discord, Telegram y Github"</p>
            <div class="flex justify-between min-w-[20rem] items-start gap-28">
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
    }

}
