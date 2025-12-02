use leptos::{prelude::*, *};
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant},
    card::Card,
    icons::{Book, Github, Project, Roadmap, StarBold},
};

use crate::{
    components::{project_card::ProjectCard, GithubIcon},
    models::GithubUser,
};

#[component]
pub fn CommunityProjectSection(
    #[prop(into, default = false)] many_projects: bool,
) -> impl IntoView {
    let juanperas = GithubUser{ username: "juanperas", avatar: "https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4" };
    let apika = GithubUser { username: "ApikaLucas", avatar: "https://avatars.githubusercontent.com/u/70247585?u=513513290efb6dc162afc899646e8e0467cedfc2&v=4"};
    let sergio = GithubUser { username: "Sergio", avatar: "https://avatars.githubusercontent.com/u/56278796?u=9e3dac947b4fd3ca2f1a05024e083c64e4c69cfe&v=4"};

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
                <div class="flex flex-row justify-center items-center gap-4 max-w-full w-full">
                    <div
                        class="inline-flex flex-row items-center gap-8 overflow-x-auto p-4"
                        class:md:flex-wrap=many_projects
                    >
                        <ProjectCard
                            label=view! {
                                <StarBold size=16 />
                                <p class="w-fit mr-2">Destacado</p>
                            }
                                .into_any()
                            users=users.clone()
                            title="Rust para C#/.NET Developers"
                            description="La guía esta hecha por la misma Microsoft y es para desarrolladores experimentados en C#/.NET que exploran Rust. Ofrece una breve comparación, enlaces a recursos y respuestas rápidas."
                        />
                        <ProjectCard
                            label=view! { <p class="w-fit mr-2">Oficial</p> }.into_any()
                            users=users.clone()
                            title="Cangrebot"
                            description="Bot de la comunidad de Discord de RustLang en Español."
                            badge_color="before:bg-secondary-400"
                        />
                        <ProjectCard
                            label=view! { <p class="w-fit mr-2">$200</p> }.into_any()
                            users=users.clone()
                            title="Memsos"
                            description="Memsos is a tool written in rust with the objective to check your memory in a fast way, memsos works for both uefi and bios."
                            badge_color="before:bg-primary-200"
                        />

                        {if many_projects {
                            view! {
                                <>
                                    <ProjectCard
                                        label=view! { <p class="w-fit mr-2">$200</p> }.into_any()
                                        users=users.clone()
                                        title="Memsos"
                                        description="Memsos is a tool written in rust with the objective to check your memory in a fast way, memsos works for both uefi and bios."
                                        badge_color="before:bg-primary-200"
                                    />
                                    <ProjectCard
                                        label=view! { <p class="w-fit mr-2">$200</p> }.into_any()
                                        users=users.clone()
                                        title="Memsos"
                                        description="Memsos is a tool written in rust with the objective to check your memory in a fast way, memsos works for both uefi and bios."
                                        badge_color="before:bg-primary-200"
                                    />
                                    <ProjectCard
                                        label=view! { <p class="w-fit mr-2">$200</p> }.into_any()
                                        users=users.clone()
                                        title="Memsos"
                                        description="Memsos is a tool written in rust with the objective to check your memory in a fast way, memsos works for both uefi and bios."
                                        badge_color="before:bg-primary-200"
                                    />
                                </>
                            }
                                .into_any()
                        } else {
                            view! { <></> }.into_any()
                        }}
                    </div>

                </div>

                <Button
                    variant=Variant::Secondary
                    label="Ver proyectos"
                    icon=view! { <Github /> }.into_any()
                    on_click=move |_| {}
                ></Button>
            </div>
        </section>
    }
}
