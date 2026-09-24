use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant as ButtonVariant},
    icons::{Discord, Github, Linkedin, Telegram},
};

use crate::components::{GITHUB_PATH, JOIN_PATH, LINKEDIN_PATH, TELEGRAM_PATH};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="w-full flex flex-col items-center justify-between bg-primary-500 dark:bg-primary-600 py-10">
            <section>
                <p class="py-3 font-semibold">"Comunidad - Rust Lang en Español"</p>
            </section>
            <div class="flex gap-4 justify-center items-center w-full">
                <a href=JOIN_PATH target="_blank" rel="noopener noreferrer">
                    <Button
                        variant=ButtonVariant::Icon
                        on_click=|_| {}
                        icon=view! { <Discord /> }.into_any()
                    />
                </a>
                <a href=GITHUB_PATH target="_blank" rel="noopener noreferrer">
                    <Button
                        variant=ButtonVariant::Icon
                        on_click=|_| {}
                        icon=view! { <Github /> }.into_any()
                    />
                </a>
                <a href=LINKEDIN_PATH target="_blank" rel="noopener noreferrer">
                    <Button
                        variant=ButtonVariant::Icon
                        on_click=|_| {}
                        icon=view! { <Linkedin /> }.into_any()
                    />
                </a>
                <a href=TELEGRAM_PATH target="_blank" rel="noopener noreferrer">
                    <Button
                        variant=ButtonVariant::Icon
                        on_click=|_| {}
                        icon=view! { <Telegram /> }.into_any()
                    />
                </a>
            </div>
        </footer>
    }
}
