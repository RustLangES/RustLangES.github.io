use leptos::{component, prelude::*};
use rustlanges_components::{
    card::{Card, Variant as CardVariant},
    chip::{Chip, Variant as ChipVariant},
    icons::{ArrowDown, File},
    input::{Filter, InputSearch},
    tag::Tag,
};

use crate::components::carousel::Carousel;

type ResourceAlias = Vec<Filter>;

#[component]
pub fn Resources() -> impl IntoView {
    view! {
        <section class="bg-light dark:bg-dark py-12 lg:p-20 w-full">
            
            <div class="container xl:max-w-7xl lg:px-36 mx-auto flex flex-col gap-6">
                <div class="px-4 flex flex-col gap-2 items-center text-center">
                    <h2 class="text-heading-2 md:text-heading-1 font-bold">"Recursos"</h2>
                    <p class="text-paragraph-1 font-semibold max-w-xs md:max-w-none">
                        "Encuentra un montón de recursos gratuitos para aprender sobre Rust"
                    </p>
                </div>

                <div class="px-4 w-full max-w-xl mx-auto">
                    <InputSearch
                        filters=["Libros", "Guías", "Videos", "Cursos"]
                            .map(|label| Filter {
                                label: label.to_string(),
                                value: label.to_lowercase(),
                            })
                            .to_vec()
                        on_change_filter=move |_resource: ResourceAlias| ()
                    />
                </div>

                <Carousel
                    count=3
                    class="md:flex-wrap md:justify-center md:overflow-visible".to_string()
                >
                    {(0..3)
                        .map(|_| {
                            view! {
                                <div class="shrink-0 snap-start w-[75%] max-w-72 md:w-72">
                                    <Card variant=CardVariant::Resource>
                                        <div class="flex flex-col gap-4 justify-between h-full w-full overflow-hidden">
                                            <div class="flex flex-col gap-2 items-start">
                                                <Chip label="Destacado" variant=ChipVariant::Featured />
                                                <h3 class="text-h4">"Título del recurso"</h3>
                                                <p class="text-paragraph-1">
                                                    "Lorem ipsum dolor sit amet consectetur."
                                                </p>
                                            </div>

                                            <div class="flex gap-2 flex-wrap">
                                                <Tag label="#tag" />
                                                <Tag label="#tag" />
                                            </div>
                                        </div>
                                    </Card>
                                </div>
                            }
                        })
                        .collect_view()}
                </Carousel>

                <details open class="group px-4 w-full max-w-3xl mx-auto">

                    <summary class="flex items-center justify-between cursor-pointer list-none [&::-webkit-details-marker]:hidden py-2">
                        <span class="text-paragraph-1">"Todos"</span>
                        <span class="flex items-center justify-center size-9 rounded-full border border-black dark:border-light bg-light dark:bg-neutral-950 transition-transform group-open:rotate-180">
                            <ArrowDown />
                        </span>
                    </summary>

                    <ul class="flex flex-col gap-3 pt-2">
                        {(0..3)
                            .map(|_| {
                                view! {
                                    <li>
                                        <Card class="flex items-center gap-3 py-3 bg-primary-50 dark:bg-neutral-900">
                                            <File class="text-primary-500 shrink-0" />
                                            <span class="text-h5">"Título del recurso"</span>
                                        </Card>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                    
                </details>
            </div>
        </section>
    }
}
