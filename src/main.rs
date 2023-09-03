mod app;
mod components;
#[rustfmt::skip]
mod extras;
mod models;
mod pages;

use app::*;
use leptos::*;
use log::info;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("Si estas leyendo esto, hace un pull request:");
    info!("https://github.com/RustLangES/RustLangES.github.io/issues");
    info!("https://youtu.be/MldLXIB_ZXE");

    mount_to_body(|| {
        view! { <App/> }
    });
}
