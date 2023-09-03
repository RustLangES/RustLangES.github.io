use leptos::*;

use crate::{components::cards::project_card::ProjectCard, extras::COMUNITY_PROJECTS};

#[component]
pub fn CommunityProjects() -> impl IntoView {
    view! {
        <section class="bg-orange-200 py-20">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl text-left mb-6">
                    <span class="font-work-sans font-light">"Proyectos de la "</span>
                    <span class="font-alfa-slab text-orange-500">"Comunidad"</span>
                </h2>
                <div class="w-full grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 sm:gap-x-8 gap-y-4">
                    {COMUNITY_PROJECTS
                        .iter()
                        .map(|item| {
                            view! {
                                <ProjectCard
                                    name=item.name
                                    description=item.description
                                    link=item.link
                                    brand_src=item.brand_src
                                    button_link=item.button_link
                                    button_text=item.button_text
                                    brand_as_letter=item.brand_as_letter
                                    button_bg_color=item.button_bg_color
                                />
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}
