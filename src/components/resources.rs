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
        <section class="bg-light dark:bg-dark p-4 lg:p-20 w-full">
            <div class="container xl:max-w-7xl lg:px-36 mx-auto">
                <h2 class="text-h2 text-center mb-8 mt-16">"Recursos"</h2>
                <div class="flex flex-col justify-center mb-8 container mx-auto max-w-fit">
                    <InputSearch on_change_filter=move |resource: ResourceAlias| () />
                    <div class="gap-6 overflow-x-auto p-4 flex flex-row flex-wrap justify-center items-center">

                        <Card
                            class="p-4 text-primary-50 dark:text-neutral-900"
                            variant=CardVariant::Resource
                        >
                            <div class="flex flex-col gap-4 justify-between h-full overflow-hidden">
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
            </div>
        </section>
    }
}
