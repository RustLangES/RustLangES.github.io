use crate::{
    components::icons::{NewLogoRustDarkPageIcon, NewLogoRustLightPageIcon},
    context::theme_provider::{Theme, use_theme},
};
use leptos::prelude::*;
use leptos_use::{use_media_query, use_window};
use rustlanges_components::{
    button::{Button, Variant},
    icons::{Close, Menu, Moon, SunLine, SunMoon},
};

/// Path that redirects to the book in spanish from RustLangEs
const BOOK_PATH: &str = "https://book.rustlang-es.org/";

/// Path that redirects to the discord community server
const JOIN_PATH: &str = "https://discord.rustlang-es.org/";

#[island]
pub fn Header() -> impl IntoView {
    let this = use_window();
    let path = RwSignal::new("/".to_string());
    let menu_open = RwSignal::new(false);

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

    let menu_handler = move |_| {
        menu_open.update(|open| *open = !*open);
    };

    let menu_icon = move || {
        if menu_open() {
            view! { <Close size=20 class="z-50 block" /> }.into_any()
        } else {
            view! { <Menu size=20 class="block" /> }.into_any()
        }
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

    let theme_handler = move |_| {
        let current_theme = theme.get();
        match current_theme {
            Theme::Light => theme.set(Theme::Dark),
            Theme::Dark | Theme::System => theme.set(Theme::Light),
        }
    };

    view! {
        <header class="w-full py-[8px] px-[0px] flex flex-column items-center justify-between md:px-[24px] relative">
            <div class="flex justify-between items-center">
                <Button
                    variant=Variant::Text
                    class="px-[10px] md:hidden flex items-center justify-center"
                    on_click=menu_handler
                    icon=(move || menu_icon()).into_any()
                />
                <div class=move || {
                    format!(
                        "md:hidden absolute top-full left-0 w-full bg-white dark:bg-dark flex flex-col gap-[32px] p-[16px] z-50 {}",
                        if menu_open() { "flex" } else { "hidden" },
                    )
                }>
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
                    <a href="/blog" class=move || active_link_class("/blog")>
                        Blog
                    </a>

                    <Button
                        variant=Variant::Icon
                        on_click=theme_handler
                        icon=(move || theme_switcher_icon()).into_any()
                    />

                    <div class="grid grid-cols-2 gap-[8px] w-full">
                        <a href=BOOK_PATH target="_blank" rel="noopener noreferrer">
                            <Button
                                class="w-full"
                                variant=Variant::Secondary
                                label="El Libro"
                                on_click=|_| {}
                            />
                        </a>
                        <a href=JOIN_PATH target="_blank" rel="noopener noreferrer">
                            <Button
                                class="w-full"
                                variant=Variant::Primary
                                label="¡Únete!"
                                on_click=move |_| {}
                            />
                        </a>
                    </div>
                    <hr />
                    <p class="text-center opacity-50">Comunidad - Rust Lang en Español</p>
                </div>
                <div
                    class=move || {
                        format!(
                            "fixed inset-0 bg-black/50 z-40 md:hidden {}",
                            if menu_open() { "block" } else { "hidden" },
                        )
                    }
                    on:click=move |_| menu_open.set(false)
                ></div>
                <a href="/" class=move || active_link_class("/")>
                    {move || logo()}
                </a>
            </div>
            <div class="flex flex-column gap-[24px] items-center">
                <nav class="gap-[16px] hidden md:flex">
                    <a href="/" class=move || format!("font-semibold {}", active_link_class("/"))>
                        Inicio
                    </a>
                    <a
                        href="/aprende"
                        class=move || format!("font-semibold {}", active_link_class("/aprende"))
                    >
                        Aprende Rust
                    </a>
                    <a
                        href="/comunidad"
                        class=move || format!("font-semibold {}", active_link_class("/comunidad"))
                    >
                        Comunidad
                    </a>
                    <a
                        href="/eventos"
                        class=move || format!("font-semibold {}", active_link_class("/eventos"))
                    >
                        Eventos
                    </a>
                    <a
                        href="/blog"
                        class=move || format!("font-semibold {}", active_link_class("/blog"))
                    >
                        Blog
                    </a>
                </nav>
                <div class="flex gap-[16px] items-center flex-wrap">
                    <a
                        href=BOOK_PATH
                        class="hidden md:flex"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <Button variant=Variant::Secondary label="El Libro" on_click=|_| {} />
                    </a>

                    <a
                        href=JOIN_PATH
                        class="px-[10px] md:px-[0px]"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <Button variant=Variant::Primary label="¡Únete!" on_click=move |_| {} />
                    </a>

                    <Button
                        variant=Variant::Icon
                        class="hidden md:flex"
                        on_click=theme_handler
                        icon=(move || theme_switcher_icon()).into_any()
                    />
                </div>
            </div>
        </header>
    }
}
