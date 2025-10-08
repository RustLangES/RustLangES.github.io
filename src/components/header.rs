use crate::{
    components::icons::{NewLogoRustDarkPageIcon, NewLogoRustLightPageIcon},
    context::theme_provider::{use_theme, Theme},
};
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_router::{components::A, hooks::use_url};
use leptos_use::{use_media_query, use_window};
use rustlanges_components::{
    button::{Button, Variant},
    icons::{Moon, SunLine, SunMoon},
};

#[island]
pub fn Header() -> impl IntoView {
    let this = use_window();
    let path = RwSignal::new("/".to_string());

    Effect::new(move |_| {
        let result = format!(
            "{:?}",
            this.as_ref().unwrap().location().pathname().unwrap()
        );

        path.set(result);
    });

    let theme = use_theme();
    let is_dark_preferred_signal = use_media_query("(prefers-color-scheme: dark)");

    let logo = move || match theme.get() {
        Theme::Dark => view! { <NewLogoRustDarkPageIcon size=60 /> }.into_any(),
        Theme::Light => view! { <NewLogoRustLightPageIcon size=60 /> }.into_any(),
        Theme::System if is_dark_preferred_signal() => {
            view! { <NewLogoRustDarkPageIcon size=60 /> }.into_any()
        }
        Theme::System => view! { <NewLogoRustLightPageIcon size=60 /> }.into_any(),
    };

    let theme_switcher_icon = move || match theme() {
        Theme::Dark => view! { <Moon /> }.into_any(),
        Theme::Light => view! { <SunLine /> }.into_any(),
        Theme::System => view! { <SunMoon /> }.into_any(),
    };

    let active_link_class = move |link: &str| {
        if path() == format!("{:?}", link) {
            "text-red-500 dark:text-orange-300"
        } else {
            ""
        }
    };

    let handler = move |_| {
        let current_theme = theme.get();
        match current_theme {
            Theme::System => theme.set(Theme::Dark),
            Theme::Dark => theme.set(Theme::Light),
            Theme::Light => theme.set(Theme::System),
        }
    };

    view! {
        <header class="w-full py-[8px] px-[24px] flex flex-column items-center justify-between">
            {move || logo()} <div class="flex flex-column gap-[24px] items-center">
                <div class="flex gap-[16px]">
                    <a href="/" class=move || active_link_class("/")>
                        Inicio
                    </a>
                    <a href="/aprende" class=move || active_link_class("/aprende")>
                        Aprende Rust
                    </a>
                    <a href="/comunidad" class=move || active_link_class("/comunidad")>
                        Comunidad
                    </a>
                    <a href="/eventos" class=move || active_link_class("/eventos")>
                        Eventos
                    </a>
                    <a href="https://blog.rustlang-es.org">Blog</a>
                </div>
                <div class="flex gap-[16px] items-center">
                    <Button variant=Variant::Secondary label="El Libro" on_click=|_| {} />
                    <Button
                        variant=Variant::Primary
                        label="¡Únete!"
                        on_click=move |_| console_log("hola")
                    />
                    <Button variant=Variant::Icon on_click=handler icon=theme_switcher_icon() />
                </div>
            </div>
        </header>
    }
}
