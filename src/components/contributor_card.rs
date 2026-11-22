use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant as ButtonVariant},
    card::Card,
    icons::{Github, Location, Twitter},
};

use crate::models::Contributor;

#[component]
pub fn ContributorCard(
    contributor: &'static Contributor,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let name = contributor.display_name();

    view! {
        <Card class=format!("h-full {class}")>
            <div class="flex flex-row items-center gap-4 h-full">
                <Avatar url=contributor.avatar alt=name size=52 class="min-w-13 shrink-0" />

                <div class="flex flex-col gap-1 min-w-0 grow">
                    <h3 class="text-h5 truncate" title=name>
                        {name}
                    </h3>

                    {contributor
                        .location
                        .map(|location| {
                            view! {
                                <div class="rustlanges-chip rustlanges-chip--location max-w-full">
                                    <Location class="shrink-0" />
                                    <span class="truncate min-w-0">{location}</span>
                                </div>
                            }
                        })}
                </div>
                <div class="flex gap-3 items-center justify-center shrink-0">

                    {contributor
                        .twitter_url()
                        .map(|url| {
                            view! {
                                <a href=url target="_blank">
                                    <Button
                                        variant=ButtonVariant::Icon
                                        icon=view! { <Twitter /> }.into_any()
                                        on_click=move |_| {}
                                    />
                                </a>
                            }
                        })}
                    <a href=contributor.github_url() target="_blank">
                        <Button
                            variant=ButtonVariant::Icon
                            icon=view! { <Github /> }.into_any()
                            on_click=move |_| {}
                        />
                    </a>
                </div>
            </div>
        </Card>
    }
}
