use leptos::prelude::*;
use rustlanges_components::{avatar::Avatar, card::Card, icons::Github};

use crate::models::GithubUser;

#[allow(unused_variables)]
#[component]
pub fn ProjectCard(
    #[prop(into)] title: &'static str,
    #[prop(into)] description: &'static str,
    #[prop()] users: Vec<GithubUser>,
    #[prop()] label: AnyView,
    #[prop(optional, default = "")] badge_color: &'static str,
    #[prop(optional)] link: Option<&'static str>,
) -> impl IntoView {
    let avatars = users
        .iter()
        .enumerate()
        .map(|(i, user)| {
            let class = if i == 0 {
                "border-black"
            } else {
                "-ml-2 border-black"
            };
            view! { <Avatar class=class url=user.avatar alt=user.username /> }
        })
        .collect_view();

    let badge_classes = format!(
        "flex w-fit project-card-badge ml-6  px-5 pb-1 pt-2 items-center justify-center gap-2 text-black border-2 border-b-0 border-black before:bg-orange-400 {badge_color}"
    );

    view! {
        <div class="min-w-[15rem] max-w-xs">
            <div class=badge_classes>{label}</div>
            <Card class="relative max-w-xs lg:max-w-md min-h-96 flex flex-col justify-between ">
                <div id="card-body-1">
                    <h3 class="text-h4">{title}</h3>
                    <p>{description}</p>
                </div>
                <div class="flex items-center justify-between [&>*]:-ml-2">
                    <div class="flex items-center justify-center ">{avatars}</div>

                    <a class="rounded-full border border-white p-1" href=link target="_blank">
                        <Github size=40 />
                    </a>
                </div>
            </Card>
        </div>
    }
}
