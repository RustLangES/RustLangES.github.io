use crate::{
    components::icons::{NewLogoRustDarkPageIcon, NewLogoRustLightPageIcon},
    context::theme_provider::{use_theme, Theme},
};
use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_use::{use_media_query, use_window};
use rustlanges_components::{
    button::{Button, Variant},
    icons::{Menu, Moon, SunLine, SunMoon},
};

fn theme_icon(theme: Theme) -> AnyView {
    match theme {
        Theme::Dark => view! { <Moon /> }.into_any(),
        Theme::Light => view! { <SunLine /> }.into_any(),
        Theme::System => view! { <SunMoon /> }.into_any(),
    }
}

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
        Theme::System if (move || is_dark_preferred_signal())() => {
            view! { <NewLogoRustDarkPageIcon size=60 /> }.into_any()
        }
        Theme::System => view! { <NewLogoRustDarkPageIcon size=60 /> }.into_any(),
    };

    let active_link_class = move |link: &str| {
        if path() == format!("{:?}", link) {
            "text-primary-500 dark:text-orange-300 font-semibold"
        } else {
            "hover:text-primary-500 dark:hover:text-orange-300 transition-colors"
        }
    };

    view! {
        <header class="w-full py-2 px-6 flex items-center justify-between relative z-40">
            <a href="/">{move || logo()}</a>

            // Desktop nav
            <div class="hidden md:flex gap-6 items-center">
                <nav class="flex gap-5 items-center font-body text-sm">
                    <a href="/" class=move || active_link_class("/")>"Inicio"</a>
                    <a href="/aprende" class=move || active_link_class("/aprende")>"Aprende Rust"</a>
                    <a href="/comunidad" class=move || active_link_class("/comunidad")>"Comunidad"</a>
                    <a href="/eventos" class=move || active_link_class("/eventos")>"Eventos"</a>
                    <a href="https://blog.rustlang-es.org" class="hover:text-primary-500 dark:hover:text-orange-300 transition-colors">"Blog"</a>
                </nav>
                <div class="flex gap-3 items-center">
                    <Button variant=Variant::Secondary label="El Libro" on_click=|_| {} />
                    <Button variant=Variant::Primary label="¡Únete!" on_click=move |_| console_log("hola") />
                    <Button
                        variant=Variant::Icon
                        on_click=move |_| {
                            theme.update(|t| *t = match *t {
                                Theme::System => Theme::Dark,
                                Theme::Dark => Theme::Light,
                                Theme::Light => Theme::System,
                            });
                        }
                        icon=(move || theme_icon(theme.get())).into_any()
                    />
                </div>
            </div>

            // Mobile: Únete + Hamburger
            <div class="flex md:hidden items-center gap-3">
                <Button variant=Variant::Primary label="¡Únete!" on_click=move |_| console_log("hola") />
                <button
                    class="p-2 rounded-xl border-2 border-black dark:border-neutral-600 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
                    on:click=move |_| menu_open.set(true)
                >
                    <Menu size=20 />
                </button>
            </div>
        </header>

        // Mobile menu overlay
        <Show when=move || menu_open.get()>
            <div class="fixed inset-0 bg-[#FAFAFA] dark:bg-[#222222] z-50 flex flex-col p-6 overflow-y-auto">
                // Close button
                <div class="flex justify-end mb-6">
                    <button
                        class="p-2 rounded-xl hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
                        on:click=move |_| menu_open.set(false)
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
                            <line x1="18" y1="6" x2="6" y2="18"/>
                            <line x1="6" y1="6" x2="18" y2="18"/>
                        </svg>
                    </button>
                </div>

                // Nav links
                <nav class="flex flex-col gap-8 flex-1 py-4">
                    <a
                        href="/"
                        on:click=move |_| menu_open.set(false)
                        class="text-2xl font-bold hover:text-primary-500 transition-colors"
                    >"Inicio"</a>
                    <a
                        href="/aprende"
                        on:click=move |_| menu_open.set(false)
                        class="text-2xl font-bold hover:text-primary-500 transition-colors"
                    >"Aprende Rust"</a>
                    <a
                        href="/comunidad"
                        on:click=move |_| menu_open.set(false)
                        class="text-2xl font-bold hover:text-primary-500 transition-colors"
                    >"Comunidad"</a>
                    <a
                        href="/eventos"
                        on:click=move |_| menu_open.set(false)
                        class="text-2xl font-bold hover:text-primary-500 transition-colors"
                    >"Eventos"</a>
                    <a
                        href="https://blog.rustlang-es.org"
                        on:click=move |_| menu_open.set(false)
                        class="text-2xl font-bold hover:text-primary-500 transition-colors"
                    >"Blog"</a>
                </nav>

                // Bottom actions
                <div class="flex flex-col gap-4 mt-8 border-t border-neutral-200 dark:border-neutral-700 pt-6">
                    <Button
                        variant=Variant::Icon
                        on_click=move |_| {
                            theme.update(|t| *t = match *t {
                                Theme::System => Theme::Dark,
                                Theme::Dark => Theme::Light,
                                Theme::Light => Theme::System,
                            });
                        }
                        icon=(move || theme_icon(theme.get())).into_any()
                    />
                    <div class="flex gap-4">
                        <Button variant=Variant::Secondary label="El Libro" on_click=|_| {} class="flex-1" />
                        <Button
                            variant=Variant::Primary
                            label="¡Únete!"
                            on_click=move |_| { menu_open.set(false); console_log("hola"); }
                            class="flex-1"
                        />
                    </div>
                </div>

                <p class="text-center text-sm text-neutral-400 dark:text-neutral-500 font-body mt-6">
                    "Comunidad - Rust Lang en Español"
                </p>
            </div>
        </Show>
    }
}
