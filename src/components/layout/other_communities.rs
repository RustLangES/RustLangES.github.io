

use leptos::*;

use crate::components::atom::discord_icon::DiscordIcon;
use crate::components::atom::github_icon::GithubIcon;
use crate::components::atom::telegram_icon::TelegramIcon;

#[component]
pub fn OtherCommunities(
    cx: Scope
) -> impl IntoView {
    view!{ cx, 
        <div class="bg-slate-300 dark:bg-zinc-900 w-full bg-no-repeat bg-center flex justify-center items-center flex-col pb-20">
            <h2 class="p-6 text-2xl">"Otras comunidades recomendadas"</h2>
            <div class="flex justify-between min-w-[20rem] gap-10 items-start">
                    <div class="flex flex-col justify-center items-center">
                        <a href="https://discord.gg/DeZKCCyMcq" target="_blank"  class="flex flex-col justify-center items-center underline text-blue-600 hover:text-blue-800 visited:text-purple-600 dark:text-sky-300 dark:hover:text-sky-200 dark:visited:text-green-200">
                        <img src="./RustBCN.webp" width="50" class="rounded-full" />
                        <p>"Rust Barcelona"</p>
                    </a>
                    <p class="text-sm">"Discord"</p>
                </div>
                <div class="flex flex-col justify-center items-center">
                    <a href="https://discord.gg/b9EbVnSkuw" target="_blank"  class="flex flex-col justify-center items-center underline text-blue-600 hover:text-blue-800 visited:text-purple-600 dark:text-sky-300 dark:hover:text-sky-200 dark:visited:text-green-200">
                        <img src="./Whizzles.webp" width="50" />
                        <p>"Whizzles"</p>
                    </a>
                    <p class="text-sm">"Discord"</p>
                </div>
                <div class="flex flex-col justify-center items-center">
                    <a href="https://t.me/RustMX" target="_blank"  class="flex flex-col justify-center items-center underline text-blue-600 hover:text-blue-800 visited:text-purple-600 dark:text-sky-300 dark:hover:text-sky-200 dark:visited:text-green-200">
                        <img src="./RustMX.png" width="50" />
                        <p>"RustMX"</p>
                    </a>
                    <p class="text-sm">"Telegram"</p>
                </div>
                <div class="flex flex-col justify-center items-center">
                    <a href="https://t.me/rust_lang_es" target="_blank"  class="flex flex-col justify-center items-center underline text-blue-600 hover:text-blue-800 visited:text-purple-600 dark:text-sky-300 dark:hover:text-sky-200 dark:visited:text-green-200">
                        <img src="./RustGeneral.jpg" width="50" class="rounded-full" />
                        <p>"Rust Español"</p>
                    </a>
                    <p class="text-sm">"Telegram"</p>
                </div>
                <div class="flex flex-col justify-center items-center">
                    <a href="https://t.me/aprenderrust" target="_blank"  class="flex flex-col justify-center items-center underline text-blue-600 hover:text-blue-800 visited:text-purple-600 dark:text-sky-300 dark:hover:text-sky-200 dark:visited:text-green-200">
                        <img src="./aprender-rust.jpg" width="50" class="rounded-full" />
                        <p>"Aprender Rust"</p>
                    </a>
                    <p class="text-sm">"Telegram"</p>
                </div>
            </div>
        </div>
    }
        
}