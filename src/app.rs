use futures::{channel::mpsc, Stream};
use leptos::{component, prelude::*, view, IntoView};
use leptos_meta::{provide_meta_context, Body};
use leptos_router::{components::*, path, static_routes::StaticRoute, SsrMode};
use std::path::Path;

use crate::{
    components::{Footer, HeadInformation, Header},
    pages::{Aprende, Communities, Contributors, Index, Projects},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <leptos_meta::MetaTags />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let bg_in_dark_mode = if cfg!(debug_assertions) {
        "dark:bg-kaku-dev"
    } else {
        "dark:bg-kaku"
    };

    view! {
        <Router>
            <HeadInformation />
            <Body
                {..}
                class=format!(
                    "bg-orange-200 dark:bg-[#131313]/90 bg-center bg-fixed {} dark:bri dark:bg-cover dark:bg-blend-darken dark:backdrop-blur-xl overflow-x-hidden dark:text-[#e2cea9]",
                    bg_in_dark_mode,
                )
            />
            <Header />
            <main>
                <FlatRoutes fallback=|| Index>
                    <Route path=path!("/") view=Index ssr=SsrMode::Static(StaticRoute::new()) />
                    <Route
                        path=path!("/comunidades")
                        view=Communities
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                    <Route
                        path=path!("/colaboradores")
                        view=Contributors
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                    <Route
                        path=path!("/proyectos")
                        view=Projects
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                    <Route
                        path=path!("/aprende")
                        ssr=SsrMode::Static(StaticRoute::new())
                        view=Aprende
                    />
                    <Route path=path!("/404") view=Index ssr=SsrMode::Static(StaticRoute::new()) />
                </FlatRoutes>
            </main>
            <Footer />
        </Router>
    }
}

#[allow(unused)] // path is not used in non-SSR
fn watch_path(path: &Path) -> impl Stream<Item = ()> {
    #[allow(unused)]
    let (mut tx, rx) = mpsc::channel(0);

    #[cfg(feature = "ssr")]
    {
        use notify::{RecursiveMode, Watcher};

        let mut watcher = notify::recommended_watcher(move |res: Result<_, _>| {
            if res.is_ok() {
                // if this fails, it's because the buffer is full
                // this means we've already notified before it's regenerated,
                // so this page will be queued for regeneration already
                _ = tx.try_send(());
            }
        })
        .expect("could not create watcher");

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher
            .watch(path, RecursiveMode::NonRecursive)
            .expect("could not watch path");

        // we want this to run as long as the server is alive
        std::mem::forget(watcher);
    }

    rx
}
