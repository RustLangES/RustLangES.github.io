use leptos::config::LeptosOptions;
use leptos::{logging::log, prelude::*};
use leptos_actix::generate_route_list_with_ssg;
use rust_lang_es::app::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
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
