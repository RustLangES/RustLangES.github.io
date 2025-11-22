use leptos::{component, prelude::*};
use rustlanges_components::{
    badge::Badge,
    card::{Card, Variant as CardVariant},
    chip::{Chip, Variant as ChipVariant},
    input::{Filter, InputSearch},
    tag::Tag,
};

type ResourceAlias = Vec<Filter>;

#[component]
pub fn Resources() -> impl IntoView {
    let (resources, set_resources) = signal(vec![Filter {
        label: "Libros".to_string(),
        value: "books".to_string(),
    }]);

    view! {
        <section class="bg-light dark:bg-dark p-20 w-full">
            <div class="container px-40 mx-auto">
                <h2 class="text-h2 text-center mb-8 mt-16">"Recursos"</h2>
                <InputSearch on_change_filter=move |resource: ResourceAlias| () />
                <div class="gap-4 overflow-x-auto p-4 grid justify-items-center grid-cols-1 md:grid-cols-2 lg:grid-cols-3">

                    <Card class="p-4" variant=CardVariant::Resource>
                        <div class="flex flex-col gap-4 justify-between h-full">
                            <div>
                                <Chip label="Destacado" variant=ChipVariant::Featured />
                                <h2 class="text-h3 mt-2 mb-2">"Título del recurso"</h2>
                                <p class="text-paragraph-2 mb-4">"Descripción del recurso"</p>
                                <p class="text-paragraph-3 mb-4">
                                    "Lorem ipsum dolor sit amet consectetur."
                                </p>
                            </div>

                            <div class="flex gap-2 flex-wrap">
                                <Tag label="#tag" />
                                <Tag label="#tag" />
                                <Tag label="#tag" />
                            </div>
                        </div>
                    </Card>

                    <Card class="p-4" variant=CardVariant::Resource>
                        <div class="flex flex-col gap-4 justify-between h-full">
                            <div>
                                <Chip label="Destacado" variant=ChipVariant::Featured />
                                <h2 class="text-h3 mt-2 mb-2">"Título del recurso"</h2>
                                <p class="text-paragraph-2 mb-4">"Descripción del recurso"</p>
                                <p class="text-paragraph-3 mb-4">
                                    "Lorem ipsum dolor sit amet consectetur."
                                </p>
                            </div>

                            <div class="flex gap-2 flex-wrap">
                                <Tag label="#tag" />
                                <Tag label="#tag" />
                                <Tag label="#tag" />
                            </div>
                        </div>
                    </Card>

                    <Card class="p-4" variant=CardVariant::Resource>
                        <div class="flex flex-col gap-4 justify-between h-full">
                            <div>
                                <Chip label="Destacado" variant=ChipVariant::Featured />
                                <h2 class="text-h3 mt-2 mb-2">"Título del recurso"</h2>
                                <p class="text-paragraph-2 mb-4">"Descripción del recurso"</p>
                                <p class="text-paragraph-3 mb-4">
                                    "Lorem ipsum dolor sit amet consectetur."
                                </p>
                            </div>

                            <div class="flex gap-2 flex-wrap">
                                <Tag label="#tag" />
                                <Tag label="#tag" />
                                <Tag label="#tag" />
                            </div>
                        </div>
                    </Card>
                </div>
            </div>
        </section>
    }
}
