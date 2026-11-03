use leptos::prelude::*;
use rustlanges_components::{
    button::{Variant, button_class},
    icons::{Github, StarBold},
};

use crate::{components::project_card::ProjectCard, extras::TOP_PROJECTS, models::GithubUser};

const BADGE_COLORS: [&str; 3] = ["", "before:bg-secondary-400", "before:bg-primary-200"];

#[component]
pub fn CommunityProjectSection(#[prop(into, default = false)] with_price: bool) -> impl IntoView {
    let juanperas = GithubUser {
        username: "juanperas",
        avatar: "https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4",
    };
    let apika = GithubUser {
        username: "ApikaLucas",
        avatar: "https://avatars.githubusercontent.com/u/70247585?u=513513290efb6dc162afc899646e8e0467cedfc2&v=4",
    };
    let sergio = GithubUser {
        username: "Sergio",
        avatar: "https://avatars.githubusercontent.com/u/56278796?u=9e3dac947b4fd3ca2f1a05024e083c64e4c69cfe&v=4",
    };

    let users = vec![juanperas, apika, sergio];

    view! {
        <section class="flex flex-col justify-center items-center w-full gap-8 py-20 bg-orange-100 dark:bg-neutral-950 bg-ferris-left">
            <div class="flex flex-col justify-center items-center gap-8 w-full">
                <h2 class="text-h2 mb-4">"Proyectos de la comunidad"</h2>
                <div class="mb-8">
                    <p class="text-center max-w-lg">
                        "Te mostramos algunos de los proyectos más relevantes."
                    </p>
                    <p class="text-center max-w-lg m">
                        Conoce los <span class="font-bold">+40</span>
                        que tenemos en nuestro repositorio
                    </p>
                </div>

                // <div>
                <div class="flex flex-row justify-center items-center gap-4 max-w-full m-auto">
                    <div
                        // class="inline-flex flex-row items-center gap-8 overflow-x-auto p-4 grid grid-cols-3"
                        class="items-center gap-8 overflow-x-auto p-4 grid grid-cols-3"
                        class:md:flex-wrap=with_price
                    >
                        {if with_price {
                            (0..3)
                                .map(|_| {
                                    view! {
                                        <ProjectCard
                                            label=view! { <p class="w-fit mr-2">$200</p> }.into_any()
                                            users=users.clone()
                                            title="Memsos"
                                            description="Memsos is a tool written in rust with the objective to check your memory in a fast way, memsos works for both uefi and bios."
                                            badge_color="before:bg-primary-200"
                                        />
                                    }
                                })
                                .collect_view()
                        } else {
                            TOP_PROJECTS
                                .iter()
                                .zip(BADGE_COLORS.iter().cycle())
                                .map(|(project, badge_color)| {
                                    let label = match project.stars {
                                        Some(stars) => {
                                            view! {
                                                <StarBold size=16 />
                                                <p class="w-fit mr-2">{stars}</p>
                                            }
                                                .into_any()
                                        }
                                        None => {
                                            view! { <p class="w-fit mr-2">Destacado</p> }.into_any()
                                        }
                                    };
                                    view! {
                                        <ProjectCard
                                            label=label
                                            users=project.contributors.to_vec()
                                            title=project.name
                                            description=project.description
                                            badge_color=*badge_color
                                            link=project.link
                                        />
                                    }
                                })
                                .collect_view()
                        }}
                    </div>

                </div>

                <a
                    class=button_class(Variant::Secondary, None)
                    href="https://github.com/orgs/RustLangES/repositories?q=sort%3Astars"
                    target="_blank"
                >
                    "Ver proyectos"
                    <Github />
                </a>
            </div>
        </section>
    }
}
