use crate::{components::icons::{NewLogoRustDarkPageIcon, NewLogoRustLightPageIcon}, context::theme_provider::{use_theme, Theme}};
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_router::{components::A, hooks::use_url};
use rustlanges_components::{button::{Button, Variant}, icons::{Moon, SunLine}};
use leptos_use::use_media_query;

#[island]
pub fn Header() -> impl IntoView {
    // let url = use_url();
    let theme = use_theme();
    let is_dark_preferred_signal = use_media_query("(prefers-color-scheme: dark)");

    let logo = move || match theme.get() {
        Theme::Dark => view! {<NewLogoRustDarkPageIcon size=60 /> }.into_any(), 
        Theme::Light => view! {<NewLogoRustLightPageIcon size=60 />}.into_any(),
        Theme::System if is_dark_preferred_signal() => view! { <NewLogoRustDarkPageIcon size=60 /> }.into_any(),
        Theme::System => view! { <NewLogoRustLightPageIcon size=60 /> }.into_any()
    };

    let theme_switcher_icon = move || match theme.get() {
        Theme::Dark => view! {<SunLine /> }.into_any(), 
        Theme::Light => view! {<Moon />}.into_any(),
        Theme::System if is_dark_preferred_signal() => view! { <SunLine/> }.into_any(),
        Theme::System => view! { <Moon /> }.into_any()
    };

    // let is_active = |link:&str| url().path() == link;
    let handler = move |_| {
        let current_theme = theme.get();
        console_log(&format!("{current_theme:?}"));
        match current_theme {
            Theme::System => theme.set(Theme::Dark),
            Theme::Dark => theme.set(Theme::Light),
            Theme::Light => theme.set(Theme::System)
        }
    };

    view! {
        <header class="w-full py-[8px] px-[24px] flex flex-column items-center justify-between">
            {move || logo()}
            // {move || url.get().path().to_string() }
            <div class="flex flex-column gap-[24px] items-center">
                <div class="flex gap-[16px]">
                    // <a href="/" class=("text-red-500 dark:text-orange-300", move || is_active("/"))>Inicio</a>
                    // <a href="/aprende" class:text-red-500=move || is_active("/aprende")>Aprende Rust</a>
                    // <a href="/comunidad" class:text-red-500=is_active("/comunidad")>Comunidad</a>
                    // <a href="/eventos" class:text-red-500=is_active("/eventos")>Eventos</a>
                    <a href="https://blog.rustlang-es.org">Blog</a>
                </div>
                <div class="flex gap-[16px] items-center">
                    <Button variant=Variant::Secondary on_click=|_| {}>El Libro</Button>
                    <Button variant=Variant::Primary on_click=move |_| console_log("hola")>"¡Únete!"</Button>
                    <Button variant=Variant::Icon on_click=handler >{move || theme_switcher_icon() }</Button>
                    <button on:click=move |_| console_log("hola") > hola </button>
                </div>
            </div>
        </header>
    }
}
