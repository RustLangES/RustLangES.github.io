use leptos::{component, prelude::*};

use crate::{
    components::{
        carousel::Carousel, resource_card::ResourceCard, resource_explorer::ResourceExplorer,
    },
    extras::RESOURCES,
};

#[component]
pub fn Resources() -> impl IntoView {
    let featured = RESOURCES.iter().filter(|r| r.featured).collect::<Vec<_>>();

    view! {
        <section class="bg-light dark:bg-dark py-12 lg:p-20 w-full">

            <div class="container xl:max-w-7xl lg:px-36 mx-auto flex flex-col gap-6">
                <div class="px-4 flex flex-col gap-2 items-center text-center">
                    <h2 class="text-heading-2 md:text-heading-1 font-bold">"Recursos"</h2>
                    <p class="text-paragraph-1 font-semibold max-w-xs md:max-w-none">
                        "Encuentra un montón de recursos gratuitos para aprender sobre Rust"
                    </p>
                </div>

                <ResourceExplorer>
                    <Carousel
                        count=featured.len()
                        class="md:flex-wrap md:justify-center md:overflow-visible".to_string()
                    >
                        {featured
                            .into_iter()
                            .map(|resource| view! { <ResourceCard resource /> })
                            .collect_view()}
                    </Carousel>
                </ResourceExplorer>
            </div>
        </section>
    }
}
