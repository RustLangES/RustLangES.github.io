#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::{config::get_configuration, logging::log};
    use leptos_axum::{generate_route_list_with_ssg, LeptosRoutes};
    use rust_lang_es::app::*;
    use tower_http::services::ServeDir;

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let (routes, static_routes) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    log!("{routes:?}");

    // Run static route generation in a LocalSet to support spawn_local
    let local = tokio::task::LocalSet::new();
    local
        .run_until(static_routes.generate(&leptos_options))
        .await;

    #[cfg(feature = "development")]
    {
        println!("listening on http://{}", addr);

        use axum::routing::get;

        let site_root = leptos_options.site_root.as_ref();
        let pkg_dir = format!("{}/pkg", site_root);

        let file_serving = ServeDir::new(site_root);

        let app = Router::new()
            .route(
                "/api/{*fn_name}",
                get(leptos_axum::handle_server_fns).post(leptos_axum::handle_server_fns),
            )
            .leptos_routes(&leptos_options, routes, {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .nest_service("/pkg", ServeDir::new(pkg_dir))
            .nest_service("/assets", file_serving.clone())
            .nest_service(
                "/favicon.ico",
                ServeDir::new(format!("{}/favicon.ico", site_root)),
            )
            .fallback(leptos_axum::file_and_error_handler(shell))
            .with_state(leptos_options);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main needed here
}
