use crate::components::icons::LogoRustPageIcon;
use leptos::prelude::*;
use leptos_router::components::A;

#[island]
pub fn Header() -> impl IntoView {
    let cb_theme = || {};

    view! {
        <header class="w-full py-[8px] px-[24px] flex flex-column items-center space-between">
            <LogoRustPageIcon />
            <div class="flex flex-column gap-[24px]">
                <div class="flex gap-[16px]">
                    <A href="/">Inicio</A>
                    <A href="/aprende">Aprende Rust</A>
                    <A href="/comunidad">Comunidad</A>
                    <A href="/eventos">Eventos</A>
                    <A href="https://blog.rustlang-es.org">Blog</A>
                </div>
                <div class="flex gap-[16px]">
                </div>
            </div>
        </header>
    }
}
