use leptos::{prelude::*, *};
use rustlanges_components::{
    button::{Button, Variant as ButtonVariant},
    icons::{Discord, Github, Linkedin, Telegram},
};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center w-full gap-8 py-10 bg-primary-500 dark:bg-primary-600">
            <div class="container flex flex-col justify-center items-center gap-3">
                <p class="text-center max-w-xl">"Comunidad - Rust Lang en Español"</p>
                <div class="flex gap-4 justify-center items-center w-full">
                    <Button
                        variant=ButtonVariant::Icon
                        on_click=|_| {}
                        icon=view! { <Discord /> }.into_any()
                    />
                    <Button
                        variant=ButtonVariant::Icon
                        on_click=|_| {}
                        icon=view! { <Github /> }.into_any()
                    />
                    <Button
                        variant=ButtonVariant::Icon
                        on_click=|_| {}
                        icon=view! { <Linkedin /> }.into_any()
                    />
                    <Button
                        variant=ButtonVariant::Icon
                        on_click=|_| {}
                        icon=view! { <Telegram /> }.into_any()
                    />
                </div>
            </div>
        </section>
    }
}
