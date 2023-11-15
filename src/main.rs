mod app;
mod components;
#[rustfmt::skip]
mod extras;
mod models;
mod pages;

use app::*;
use leptos::{logging::log, *};
use log::{error, info};
use web_sys::Url;

static API_URL: &str = "https://rust-lang-en-espanol-api.shuttleapp.rs";

pub fn main() {
    let _ = create_local_resource(move || (), |_| track_previous_url());

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("Si estas leyendo esto, hace un pull request:");
    info!("https://github.com/RustLangES/RustLangES.github.io/issues");
    info!("https://youtu.be/MldLXIB_ZXE");

    mount_to_body(|| {
        view! { <App/> }
    });
}

pub async fn track_previous_url() {
    let previous_domain = if document().referrer() == "" {
        let mut res = "Undefined".to_owned();
        if let Some(from) = document().location() {
            if let Ok(url) = Url::new(&from.href().unwrap_or_default()) {
                let search_params = url.search_params();
                res = search_params.get("from").unwrap_or("Undefined".to_string());
            }
        }
        res
    } else {
        let address = document().referrer();
        let url = Url::new(&address).unwrap();
        url.host()
    };

    let endpoint = format!("{API_URL}/track/count?reference={previous_domain}");
    match reqwasm::http::Request::post(&endpoint).send().await {
        Ok(_) => log!("Tracking previous url"),
        Err(_) => error!("Error tracking previous url"),
    };
}
