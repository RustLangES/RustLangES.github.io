use leptos::{component, prelude::*};
use rustlanges_components::{
    chip::{Chip, Variant as ChipVariant},
    tag::Tag,
};

use crate::models::ResourceItem;

#[component]
pub fn ResourceCard(resource: &'static ResourceItem) -> impl IntoView {
    let language = if resource.language == "es" {
        "#español"
    } else {
        "#inglés"
    };

    view! {
        <a
            href=resource.link
            target="_blank"
            rel="noopener noreferrer"
            class="group flex flex shrink-0 snap-start w-[85%] max-w-80 md:max-w-none md:w-[calc((100%-1rem)/2)] lg:w-[calc((100%-2rem)/3)]"
        >
            <div class="relative w-full min-h-96 lg:min-h-104 drop-shadow-[4px_4px_0_#000] group-hover:drop-shadow-[4px_4px_0_var(--color-primary-500)]">

                <div
                    class="absolute inset-0 rounded-3xl bg-black group-hover:bg-primary-500"
                    style:clip-path="polygon(0 0, calc(100% - 56px) 0, 100% 62px, 100% 100%, 0 100%)"
                />
                <div
                    class="absolute inset-0.5 rounded-[22px] bg-primary-50 dark:bg-neutral-900"
                    style:clip-path="polygon(0 0, calc(100% - 54.9px) 0, 100% 60.8px, 100% 100%, 0 100%)"
                />

                <div class="relative flex flex-col gap-4 justify-between h-full px-8 py-12 lg:px-10 lg:py-14">
                    <div class="flex flex-col gap-2 items-start">
                        {resource
                            .featured
                            .then(|| {
                                view! { <Chip label="Destacado" variant=ChipVariant::Featured /> }
                            })} <h3 class="text-h4">{resource.name}</h3>
                        <p class="text-paragraph-1">{resource.description}</p>
                    </div>

                    <div class="flex gap-2 flex-wrap">
                        {resource
                            .kinds
                            .iter()
                            .map(|kind| view! { <Tag label=format!("#{kind}") /> })
                            .collect_view()} <Tag label=language />
                    </div>
                </div>
            </div>
        </a>
    }
}
