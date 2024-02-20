use leptos::{component, view, IntoView};
use std::collections::HashMap;

use crate::components::{button_link::ButtonLink, cards::card_title::CardTitle, icons::GithubIcon};

#[component]
pub fn ProjectCard(
    #[prop(into)] link: &'static str,
    #[prop(into)] name: &'static [&'static str],
    #[prop(into)] description: &'static str,
    #[prop(into)] brand_src: String,
    #[prop(into)] button_link: &'static str,
    #[prop(into, optional)] brand_as_letter: bool,
    #[prop(into, default = "white")] button_bg_color: &'static str,
    #[prop(into, optional)] button_text: &'static str,
) -> impl IntoView {
    let colors = HashMap::from([
        ("white", "bg-white text-black"),
        ("black", "bg-black text-white"),
    ]);
    let current_color = (*colors.get(&button_bg_color).unwrap()).to_string();

    view! {
        <div class="group flex flex-col gap-y-6 border border-black p-2 sm:p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between">
            <a href=link target="_blank">

                <div class="flex flex-col justify-between gap-y-2">
                    {if brand_as_letter {
                        view! {
                            <span class=format!(
                                "h-[60px] w-[60px] rounded-full text-4xl flex justify-center items-center {}",
                                current_color,
                            )>

                                {brand_src}
                            </span>
                        }
                            .into_any()
                    } else {
                        view! {
                            <img
                                src=brand_src
                                loading="lazy"
                                width="60"
                                class=format!("rounded-full h-[60px] w-[60px] {}", current_color)
                                alt=name.join(" ")
                            />
                        }
                            .into_any()
                    }}
                    <CardTitle texts=name/>
                    <p class="mt-2 font-work-sans text-black">{description}</p>
                </div>
                <div class="flex gap-2 items-center mt-4">
                    <ButtonLink href=button_link size="tiny">
                        {if button_text.is_empty() {
                            name.join("")
                        } else {
                            button_text.to_string()
                        }}

                    </ButtonLink>
                    <span class="ml-auto">
                        <GithubIcon size=30/>
                    </span>
                </div>
            </a>
        </div>
    }
}
