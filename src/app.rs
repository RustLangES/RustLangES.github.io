use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;

use crate::components::HeadInformation;
use crate::pages::{Aprende, Communities, Contributors, Index, Projects};

#[cfg(debug_assertions)]
const ASSETS_FOLDER: &str = "./assets";

#[cfg(not(debug_assertions))]
const ASSTES_FOLDERS: &str = ".";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                <meta name="theme-color" media="(prefers-color-scheme: light)" content="#fed7aa"/>
                <meta name="theme-color" media="(prefers-color-scheme: dark)" content="#181811"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>

                <script type="text/javascript">
                    (function(c,l,a,r,i,t,y){
                        if ("localhost0.0.0.0::0192.168.0.1192.168.1.1".includes(document.location.hostname)) return;
                        c[a]=c[a]||function(){(c[a].q=c[a].q||[]).push(arguments)};
                        t=l.createElement(r);t.async=1;t.src="https://www.clarity.ms/tag/"+i;
                        y=l.getElementsByTagName(r)[0];y.parentNode.insertBefore(t,y);
                    })(window, document, "clarity", "script", "n5sqsldiw7");
                </script>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Router>
            <HeadInformation/>
            // <Body class=format!(
            //     "bg-orange-200 dark:bg-[#131313]/90 bg-center bg-fixed {} dark:bri dark:bg-cover dark:bg-blend-darken dark:backdrop-blur-xl overflow-x-hidden dark:text-[#e2cea9]",
            //     bg_in_dark_mode,
            // )/>
            // <Header/>
            <main>
                <Routes fallback=|| "Not found.">
                   <Route
                       path=path!("/")
                       view=Index
                   />
                   <Route
                       path=path!("comunidades")
                       view=Communities
                   />
                   <Route
                       path=path!("colaboradores")
                       view=Contributors
                   />
                   <Route
                       path=path!("proyectos")
                       view=Projects
                   />
                   <Route
                       path=path!("aprende")
                       view=Aprende
                   />
                </Routes>
            </main>
            // <Footer />
        </Router>
    }
}
