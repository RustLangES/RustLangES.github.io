use crate::{
    components::icons::{NewLogoRustDarkPageIcon, NewLogoRustLightPageIcon},
    context::theme_provider::{Theme, use_theme},
};
use leptos::prelude::*;
use leptos_use::{use_media_query, use_window};
use rustlanges_components::{
    button::{Button, Variant},
    icons::{Moon, SunLine, SunMoon},
};

/// Path that redirects to the book in spanish from RustLangEs
const BOOK_PATH: &str = "https://book.rustlang-es.org/";

/// Path that redirects to the discord community server
const JOIN_PATH: &str = "https://discord.rustlang-es.org/";

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
        Theme::System => view! { <NewLogoRustDarkPageIcon size=60 /> }.into_any(),
    };

    let theme_switcher_icon = move || match theme() {
        Theme::Dark => view! { <Moon /> }.into_any(),
        Theme::Light => view! { <SunLine /> }.into_any(),
        Theme::System => view! { <SunMoon /> }.into_any(),
    };

    let active_link_class = move |link: &str| {
        if path() == format!("{link:?}") {
            "font-bold text-red-500 dark:text-orange-300"
        } else {
            ""
        }
    };

    let handler = move |_| {
        let current_theme = theme.get();
        match current_theme {
            Theme::Light => theme.set(Theme::Dark),
            Theme::Dark | Theme::System => theme.set(Theme::Light),
        }
    };

    view! {
        <header class="w-full py-[8px] px-[24px] flex flex-column items-center justify-between">
            <a href="/" class=move || active_link_class("/")>
                {move || logo()}
            </a>
            <div class="flex flex-column gap-[24px] items-center">
                <div class="gap-[16px] hidden md:flex">
                    <a href="/" class=move || format!("font-semibold {}", active_link_class("/"))>
                        Inicio
                    </a>
                    <a href="/aprende" class=move || format!("font-semibold {}", active_link_class("/aprende"))>
                        Aprende Rust
                    </a>
                    <a href="/comunidad" class=move || format!("font-semibold {}", active_link_class("/comunidad"))>
                        Comunidad
                    </a>
                    <a href="/eventos" class=move || format!("font-semibold {}", active_link_class("/eventos"))>
                        Eventos
                    </a>
                    <a href="/blog" class=move || format!("font-semibold {}", active_link_class("/blog"))>
                        Blog
                    </a>
                </div>
                <div class="flex gap-[16px] items-center flex-wrap">
                    <a
                        href=BOOK_PATH
                        class="hidden md:block"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <Button variant=Variant::Secondary label="El Libro" on_click=|_| {} />
                    </a>

                    <a
                        href=JOIN_PATH
                        class="hidden md:block"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <Button variant=Variant::Primary label="¡Únete!" on_click=move |_| {} />
                    </a>

                    <Button
                        variant=Variant::Icon
                        on_click=handler
                        icon=(move || theme_switcher_icon()).into_any()
                    />
                </div>
            </div>
        </header>
    }
}
