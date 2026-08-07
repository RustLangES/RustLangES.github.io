#[cfg(feature = "ssr")]
use leptos::{config::LeptosOptions, logging::log, prelude::*};
#[cfg(feature = "ssr")]
use leptos_actix::generate_route_list_with_ssg;
#[cfg(feature = "ssr")]
use rust_lang_es::app::*;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let (routes, static_routes) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    static_routes.generate(&leptos_options).await;

    log!("{routes:?}");

    #[cfg(feature = "development")]
    {
        use actix_files::Files;
        use actix_web::web;
        use leptos_actix::LeptosRoutes;

        let addr = leptos_options.site_addr;
        println!("listening on http://{addr}");

        return actix_web::HttpServer::new(move || {
            let site_root = leptos_options.site_root.as_ref();

            actix_web::App::new()
                .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                // serve JS/WASM/CSS from `pkg`
                .service(Files::new("/pkg", format!("{site_root}/pkg")))
                // serve other assets from the `assets` directory
                .service(Files::new("/assets", site_root))
                // serve the favicon from /favicon.ico
                .service(favicon)
                .app_data(leptos_options.clone())
                .leptos_routes(routes.to_owned(), {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                })
                .app_data(web::Data::new(leptos_options.to_owned()))
        })
        .bind(&addr)?
        .run()
        .await;
    }
    #[cfg(not(feature = "development"))]
    Ok(())
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(feature = "ssr"))]
fn main() {
    eprintln!(
        "\n\
         error: the `ssr` feature is required for the server binary.\n\
         \n\
         This project separates the WASM frontend (lib) from the Actix SSR server (bin).\n\
         `cargo run` alone builds without `ssr` and cannot resolve actix/leptos_actix.\n\
         \n\
         Do one of:\n\
           cargo run --features ssr\n\
           cargo run --features ssr,development   # dev server (same as `cargo make serve`)\n\
           cargo leptos watch --features development --hot-reload\n\
           cargo make serve                       # recommended (handles CSS + assets)\n\
         \n\
         Fresh clone? Run once:\n\
           cargo make setup   # git submodules + pnpm install + styles build\n\
        "
    );
    std::process::exit(101);
}
