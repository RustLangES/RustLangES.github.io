use leptos::{component, prelude::*};
use rustlanges_components::{
    card::Card,
    input::{Filter, InputSearch},
};

type ResourceAlias = Vec<Filter>;

#[component]
pub fn Resources() -> impl IntoView {
    let (resources, set_resources) = signal(vec![Filter {
        label: "Libros".to_string(),
        value: "books".to_string(),
    }]);

    view! {
        <section class="bg-light dark:bg-dark flex justify-center w-full">
            <div class="container">
                <h2 class="text-h2 text-center mb-8 mt-16">"Recursos"</h2>
                <InputSearch on_change_filter=move |resource: ResourceAlias| () />
                <div class="flex flex-row flex-nowrap gap-4 overflow-x-auto pb-8 pt-4 grid grid-cols-3">
                    <Card class="resource-card min-w-20 min-h-20 p-4">
                        <p>"Recurso 1"</p>
                    </Card>
                    <Card class="resource-card min-w-20 min-h-20 p-4">
                        <p>"Recurso 1"</p>
                    </Card>
                    <Card class="resource-card min-w-20 min-h-20 p-4">
                        <p>"Recurso 1"</p>
                    </Card>
                    <Card class="resource-card min-w-20 min-h-20 p-4">
                        <p>"Recurso 1"</p>
                    </Card>
                </div>
            </div>
        </section>
    }
}
