use leptos::prelude::*;
use rustlanges_components::card::Card;

#[component]
pub fn PhotoCard(
    #[prop(into)] src: String,
    #[prop(into)] description: String,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    view! {
        <Card class=format!("flex flex-col gap-4 pb-5 {class}")>
            <img src=src class="w-full rounded-lg border border-black object-cover" />
            <p class="font-monospace-body text-lg text-center">{description}</p>
        </Card>
    }
}
