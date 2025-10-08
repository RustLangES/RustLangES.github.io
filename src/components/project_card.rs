use leptos::{children::Children, prelude::*, *};
use rustlanges_components::{avatar::Avatar, card::Card, icons::Github};

use crate::{components::GithubIcon, models::GithubUser};

#[component]
pub fn ProjectCard(
    #[prop(into)] title: &'static str,
    #[prop(into)] description: &'static str,
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

    let badge_classes = format!("flex w-fit project-card-badge ml-6  px-5 pb-1 pt-2 items-center justify-center gap-2 text-black border-2 border-b-0 border-black before:bg-orange-400 {badge_color}");

    view! {
        <div>
            <div class=badge_classes>{label}</div>
            <Card class="relative max-w-xs lg:max-w-md min-h-96 flex flex-col justify-between ">
                <div id="card-body-1">
                    <h3 class="text-h4">"Rust para C#/.NET Developers"</h3>
                    <p>
                        "La guía esta hecha por la misma Microsoft y es para desarrolladores experimentados en C#/.NET que exploran Rust. Ofrece una breve comparación, enlaces a recursos y respuestas rápidas."
                    </p>
                </div>
                <div class="flex items-center justify-between [&>*]:-ml-2">
                    <div class="flex items-center justify-center ">
                        <Avatar class="border-black" url=users[0].avatar alt=users[0].username />
                        {avatars}
                    </div>

                    <div class="rounded-full border border-white p-1">
                        // <Button>
                        <Github size=40 as u32 />
                    </div>
                </div>
            </Card>
        </div>
    }
}
