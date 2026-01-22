use leptos::{component, prelude::*, view, IntoView};
use leptos_meta::{Html, Link, Meta, Stylesheet, Title};
use leptos_router::hooks::use_location;

#[component]
pub fn HeadInformation() -> impl IntoView {
    let assets_folder = if cfg!(debug_assertions) {
        "./assets"
    } else {
        "."
    };

    let location = use_location();
    let path = location.pathname.get_untracked();

    let preview = if path == "/aprende" {
        "aprende_preview.webp"
    } else {
        "rustlanges_preview.webp"
    };

    let og_image = format!("https://rustlang-es.org/{}", preview);
    let canonical_url = format!("https://rustlang-es.org{}", path);

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::web_sys;
            use wasm_bindgen::JsCast;

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let hostname = window.location().hostname().unwrap_or_default();
                    let excluded_hosts = [
                        "localhost",
                        "0.0.0.0",
                        "127.0.0.1",
                        "192.168.0.1",
                        "192.168.1.1",
                    ];

                    if !excluded_hosts.iter().any(|h| hostname.contains(h)) {
                        let script_content = r#"(function(c,l,a,r,i,t,y){
                            c[a]=c[a]||function(){(c[a].q=c[a].q||[]).push(arguments)};
                            t=l.createElement(r);t.async=1;t.src="https://www.clarity.ms/tag/"+i;
                            y=l.getElementsByTagName(r)[0];y.parentNode.insertBefore(t,y);
                        })(window, document, "clarity", "script", "n5sqsldiw7");"#;

                        if let Ok(script_element) = document.create_element("script") {
                            let script_el =
                                script_element.dyn_into::<web_sys::HtmlScriptElement>().ok();
                            if let Some(script) = script_el {
                                let _ = script.set_attribute("type", "text/javascript");
                                script.set_text_content(Some(script_content));
                                if let Some(head) = document.head() {
                                    let _ = head.append_child(&script);
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    view! {
        <>
            <Html {..} lang="es" />
            <Meta charset="utf-8" />
            <Meta name="viewport" content="width=device-width, initial-scale=1" />
            // <Meta name="theme-color" media="(prefers-color-scheme: light)" content="#fed7aa" />
            // <Meta name="theme-color" media="(prefers-color-scheme: dark)" content="#181811" />
            <Stylesheet id="fonts" href=format!("{}/fonts.css", assets_folder) />
            <Stylesheet id="leptos" href="/pkg/leptos_start.css" />
            <Title text="Rust Lang en Español" />
            <Meta
                name="description"
                content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
            />

            <Meta name="og:site_name" content="Rust Lang en Español" />
            <Meta name="og:title" content="Bienvenidos a Rust Lang en Español" />
            <Meta
                name="og:description"
                content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
            />
            <Meta name="og:url" content="https://rustlang-es.org" />

            <Meta name="twitter:card" content="summary_large_image" />
            <Meta name="twitter:site" content="@rustlang" />
            <Meta
                name="google-site-verification"
                content="OntIe2SKuQalaapGvxdded9tU4G2p57h0A6e0Rkoni0"
            />

            <Meta name="og:image" content=og_image.clone() />
            <Meta name="twitter:image" content=og_image />
            <Link rel="canonical" href=canonical_url />
        </>
    }
}
