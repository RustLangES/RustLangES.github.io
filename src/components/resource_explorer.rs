use leptos::{html, prelude::*};
use rustlanges_components::{
    card::Card,
    icons::{ArrowDown, File},
    input::{Filter, InputSearch},
};

use crate::{components::resource_card::ResourceCard, extras::RESOURCES};

#[island]
pub fn ResourceExplorer(children: Children) -> impl IntoView {
    let query: RwSignal<String> = RwSignal::new("".to_string());
    let active_filters: RwSignal<Vec<Filter>> = RwSignal::new(vec![]);
    let details = NodeRef::<html::Details>::new();

    let searching = move || !query.read().trim().is_empty();
    let results = Memo::new(move |_| {
        let query = query.read();
        RESOURCES
            .iter()
            .filter(|r| {
                query.trim().is_empty()
                    || r.name.contains(query.as_str())
                    || r.description.contains(query.as_str())
            })
            .collect::<Vec<_>>()
    });
    let cards = move || {
        if searching() {
            results.get().into_iter().take(3).collect()
        } else {
            Vec::new()
        }
    };
    let rest = move || {
        if searching() {
            results.get().into_iter().skip(3).collect()
        } else {
            results.get()
        }
    };

    Effect::new(move |_| {
        if searching()
            && let Some(el) = details.get()
        {
            el.set_open(true);
        }
    });

    let ignore_filters = |_: Vec<Filter>| ();

    view! {
        <div class="px-4 w-full" on:input=move |ev| query.set(event_target_value(&ev))>
            <InputSearch on_change_filter=ignore_filters />
        </div>

        <div class:hidden=searching>{children()}</div>

        <div
            class="flex flex-wrap justify-center gap-4 p-4"
            class:hidden=move || cards().is_empty()
        >
            <For each=cards key=|resource| resource.link let:resource>
                <ResourceCard resource />
            </For>
        </div>

        <Show when=move || results.read().is_empty()>
            <p class="px-4 text-paragraph-1 text-center py-3">"No se encontraron recursos"</p>
        </Show>

        <details node_ref=details class="group px-4 w-full" class:hidden=move || rest().is_empty()>

            <summary class="flex flex items-center justify-between cursor-pointer list-none [&::-webkit-details-marker]:hidden py-2">
                <span class="text-paragraph-1">"Todos"</span>
                <span class="flex items-center justify-center size-9 rounded-full border border-black dark:border-light bg-light dark:bg-neutral-950 transition-transform group-open:rotate-180">
                    <ArrowDown />
                </span>
            </summary>

            <ul class="grid gap-4 md:grid-cols-2 lg:grid-cols-3 pt-2">
                <For each=rest key=|resource| resource.link let:resource>
                    <li>
                        <a
                            href=resource.link
                            target="_blank"
                            rel="noopener"
                            referrerpolicy="unsafe-url"
                            class="block h-full"
                        >
                            <Card class="flex items-center gap-3 h-full py-4 bg-primary-50 dark:bg-neutral-900">
                                <File class="text-primary-500 shrink-0" />
                                <span class="text-h4">{resource.name}</span>
                            </Card>
                        </a>
                    </li>
                </For>
            </ul>

        </details>
    }
}
