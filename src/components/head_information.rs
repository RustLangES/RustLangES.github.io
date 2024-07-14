use leptos::{component, view, IntoView, SignalGet};
use leptos_meta::{Html, Link, Meta, Stylesheet, Title};
use leptos_router::use_location;

#[component]
pub fn HeadInformation() -> impl IntoView {
    let assets_folder = if cfg!(debug_assertions) {
        "./assets"
    } else {
        "."
    };

    let location = use_location().pathname.get();
    let path = location.as_str();

    let preview = if path == "/aprende" {
        "aprende_preview.webp"
    } else {
        "rustlanges_preview.webp"
    };

    view! {
        <>
            <Html lang="es"/>
            <Meta charset="utf-8"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1"/>
            <Stylesheet id="fonts" href=format!("{}/fonts.css", assets_folder)/>
            <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
            <Title text="Rust Lang en Español"/>
            <Meta
                name="description"
                content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
            />

            <Meta name="og:site_name" content="Rust Lang en Español"/>
            <Meta name="og:title" content="Bienvenidos a Rust Lang en Español"/>
            <Meta
                name="og:description"
                content="Únete a nuestra comunidad de Rust en español. Promovemos el aprendizaje y la difusión de Rust, compartiendo conocimientos y proyectos emocionantes."
            />
            <Meta name="og:url" content="https://rustlang-es.org"/>

            <Meta name="twitter:card" content="summary_large_image"/>
            <Meta name="twitter:site" content="@rustlang"/>
            <Meta
                name="google-site-verification"
                content="OntIe2SKuQalaapGvxdded9tU4G2p57h0A6e0Rkoni0"
            />

            <Meta name="og:image" content=format!("https://rustlang-es.org/{preview}")/>
            <Meta name="twitter:image" content=format!("https://rustlang-es.org/{preview}")/>
            <Link rel="canonical" href=format!("https://rustlang-es.org{}", path)/>
            <script type="text/javascript">
                (function(c,l,a,r,i,t,y){
                    if ("localhost0.0.0.0::0192.168.0.1192.168.1.1".includes(document.location.hostname)) return;
                    c[a]=c[a]||function(){(c[a].q=c[a].q||[]).push(arguments)};
                    t=l.createElement(r);t.async=1;t.src="https://www.clarity.ms/tag/"+i;
                    y=l.getElementsByTagName(r)[0];y.parentNode.insertBefore(t,y);
                })(window, document, "clarity", "script", "n5sqsldiw7");
            </script>
        </>
    }
}
