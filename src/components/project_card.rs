use leptos::prelude::*;
use rustlanges_components::{avatar::Avatar, card::Card, icons::Github};

use crate::models::GithubUser;

#[component]
pub fn ProjectCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop()] users: Vec<GithubUser>,
    #[prop()] label: AnyView,
    #[prop(optional, default = "")] badge_color: &'static str,
) -> impl IntoView {
    let avatars = users
        .iter()
        .skip(1)
        .map(|user| {
            view! { <Avatar class="-ml-2 border-black" url=user.avatar alt=user.username /> }
        })
        .collect_view();

    let badge_classes = format!(
        "flex w-fit project-card-badge ml-6 px-5 pb-1 pt-2 items-center justify-center gap-2 text-black border-2 border-b-0 border-black before:bg-orange-400 {badge_color}"
    );

    view! {
        <div class="min-w-[15rem] max-w-xs">
            <div class=badge_classes>{label}</div>
            <Card class="relative max-w-xs lg:max-w-md min-h-72 flex flex-col justify-between gap-4">
                <div>
                    <h3 class="text-h4 font-bold mb-2">{title}</h3>
                    <p class="text-sm leading-relaxed line-clamp-4">{description}</p>
                </div>
                <div class="flex items-center justify-between">
                    <div class="flex items-center [&>*:not(:first-child)]:-ml-2">
                        <Avatar class="border-black" url=users[0].avatar alt=users[0].username />
                        {avatars}
                    </div>
                    <div class="rounded-full border border-neutral-300 dark:border-neutral-600 p-2 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors cursor-pointer">
                        <Github size=20 />
                    </div>
                </div>
            </Card>
        </div>
    }
}
