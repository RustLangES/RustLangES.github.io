use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant},
    icons::{Github, StarBold},
};

use crate::{components::project_card::ProjectCard, models::GithubUser};

#[component]
pub fn CommunityProjectSection(
    #[prop(into, default = false)] many_projects: bool,
) -> impl IntoView {
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
            <div class="container max-w-7xl mx-auto px-6 flex flex-col justify-center items-center gap-8 w-full">
                <div class="flex flex-col items-center gap-3">
                    <h2 class="text-h2">"Proyectos de la comunidad"</h2>
                    <p class="text-center max-w-lg">
                        "Te mostramos algunos de los proyectos más relevantes."
                    </p>
                    <p class="text-center max-w-lg">
                        "Conoce los " <span class="font-bold">"+40"</span>
                        " que tenemos en nuestro repositorio"
                    </p>
                </div>

                <div class=move || {
                    if many_projects {
                        "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 w-full justify-items-center"
                    } else {
                        "flex flex-row gap-6 overflow-x-auto pb-4 w-full flex-wrap justify-center"
                    }
                }>
                    <ProjectCard
                        label=view! {
                            <StarBold size=16 />
                            <p class="w-fit mr-2">"Destacado"</p>
                        }
                        .into_any()
                        users=users.clone()
                        title="Rust para C#/.NET Developers"
                        description="La guía está hecha por la misma Microsoft y es para desarrolladores experimentados en C#/.NET que exploran Rust. Ofrece una breve comparación, enlaces a recursos y respuestas rápidas."
                    />
                    <ProjectCard
                        label=view! { <p class="w-fit mr-2">"Oficial"</p> }.into_any()
                        users=users.clone()
                        title="Cangrebot"
                        description="Bot de la comunidad de Discord de RustLang en Español."
                        badge_color="before:bg-secondary-400"
                    />
                    <ProjectCard
                        label=view! { <p class="w-fit mr-2">"$200"</p> }.into_any()
                        users=users.clone()
                        title="Memsos"
                        description="Memsos es una herramienta escrita en Rust para revisar tu memoria de forma rápida, funciona tanto para UEFI como para BIOS."
                        badge_color="before:bg-primary-200"
                    />

                    {many_projects.then(|| view! {
                        <ProjectCard
                            label=view! { <p class="w-fit mr-2">"Oficial"</p> }.into_any()
                            users=users.clone()
                            title="Blog RustLangES"
                            description="Blog oficial de la comunidad con artículos, tutoriales y noticias sobre Rust en español."
                            badge_color="before:bg-secondary-400"
                        />
                        <ProjectCard
                            label=view! { <p class="w-fit mr-2">"$200"</p> }.into_any()
                            users=users.clone()
                            title="Curso de Rust"
                            description="Curso completo de Rust en español para principiantes hasta nivel avanzado."
                            badge_color="before:bg-primary-200"
                        />
                        <ProjectCard
                            label=view! {
                                <StarBold size=16 />
                                <p class="w-fit mr-2">"Destacado"</p>
                            }
                            .into_any()
                            users=users.clone()
                            title="Design System Components"
                            description="Librería de componentes del sistema de diseño de RustLangES para Leptos."
                        />
                    })}
                </div>

                <Button
                    variant=Variant::Secondary
                    label="Ver proyectos"
                    icon=view! { <Github /> }.into_any()
                    on_click=move |_| {}
                />
            </div>
        </section>
    }
}
