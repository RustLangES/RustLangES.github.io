use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use crate::context::theme_provider::ThemeProvider;

use crate::components::{HeadInformation, Header};
use crate::pages::{Aprende, Communities, Contributors, Index, Projects};

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
                <HydrationScripts options islands=true/>
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
use leptos_meta::Body;
// use leptos::html::Body;
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <ThemeProvider>
            <Router>
                <HeadInformation/>
                <Body {..} class="bg-[#FAFAFA] dark:bg-[#222222] text-[#222222] dark:text-[#FAFAFA]".to_string() />
                <main>
                <Header />
                <Routes fallback=|| "Not found.">
                    <Route
                        path=path!("/")
                        view=Index
                    />
                    // <Route
                    //     path=path!("comunidades")
                    //     view=Communities
                    // />
                    // <Route
                    //     path=path!("colaboradores")
                    //     view=Contributors
                    // />
                    // <Route
                    //     path=path!("proyectos")
                    //     view=Projects
                    // />
                    // <Route
                    //     path=path!("aprende")
                    //     view=Aprende
                    // />
                    </Routes>
                    </main>
                // <Footer />
            </Router>
        </ThemeProvider>
    }
}
