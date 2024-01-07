use leptos_actix::generate_route_list_with_ssg;
use leptos_router::build_static_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use leptos::*;
    use test_leptos::app::*;

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let (routes, static_data_map) = generate_route_list_with_ssg(App);

    build_static_routes(&leptos_options, App, &routes, &static_data_map)
        .await
        .unwrap();

    #[cfg(feature = "development")]
    {
        use actix_files::Files;
        use actix_web::web;
        use leptos_actix::LeptosRoutes;

        let addr = leptos_options.site_addr.clone();
        println!("listening on http://{}", &addr);

        return actix_web::HttpServer::new(move || {
            let site_root = &leptos_options.site_root;

            actix_web::App::new()
                .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                // serve JS/WASM/CSS from `pkg`
                .service(Files::new("/pkg", format!("{site_root}/pkg")))
                // serve other assets from the `assets` directory
                .service(Files::new("/assets", site_root))
                // serve the favicon from /favicon.ico
                .service(favicon)
                .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
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
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}
