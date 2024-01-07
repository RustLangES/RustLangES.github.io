use leptos::*;
use leptos_router::*;

use crate::{components::{ProjectCard, NextIcon}, extras::COMUNITY_PROJECTS};

#[component]
pub fn CommunityProjects(#[prop(default = false)] show_more: bool) -> impl IntoView {
    view! {
        <section class="bg-orange-100 py-20">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl text-left mb-6">
                    <span class="font-work-sans font-light">"Proyectos de la "</span>
                    <span class="font-alfa-slab text-orange-500">"Comunidad"</span>
                </h2>
                <div class="w-full grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 sm:gap-x-8 gap-y-8">
                    {COMUNITY_PROJECTS
                        .iter()
                        .take(if show_more { 4 } else { COMUNITY_PROJECTS.len() })
                        .map(|item| {
                            let image_src = if cfg!(debug_assertions)
                                && item.brand_src.starts_with("/gen_assets")
                            {
                                format!("/assets{}", item.brand_src)
                            } else {
                                item.brand_src.to_string()
                            };
                            view! {
                                <ProjectCard
                                    name=item.name
                                    description=item.description
                                    link=item.link
                                    brand_src=image_src
                                    button_link=item.button_link
                                    button_text=item.button_text
                                    brand_as_letter=item.brand_as_letter
                                    button_bg_color=item.button_bg_color
                                />
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
                {if show_more {
                    view! {
                        <div class="w-full flex justify-end my-3">
                            <A
                                href="/comunidad"
                                class="text-black/80 hover:text-orange-500 fill-black/80 hover:fill-orange-500 font-work-sans font-light text-2xl flex justify-center items-center"
                            >
                                Ver todos los proyectos
                                <span class="inline-block ml-2">
                                    <NextIcon class="fill-current" size=20/>
                                </span>
                            </A>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }}

            </div>
        </section>
    }
}