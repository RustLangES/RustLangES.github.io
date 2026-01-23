use crate::components::{button_link::ButtonLink, cards::card_title::CardTitle, icons::GithubIcon};
use leptos::{component, leptos_dom::helpers::window, prelude::*, view, IntoView};
use std::collections::HashMap;

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
        ("white", "bg-white dark:bg-white text-black dark:text-black"),
        ("black", "bg-black dark:bg-white text-white dark:text-black"),
    ]);
    let current_color = (*colors
        .get(&button_bg_color)
        .expect("Button background color not found"))
    .to_string();

    view! {
        <div class="group flex flex-col items-center sm:items-stretch  gap-y-6 border border-black p-4  sm:p-6 hover:bg-orange-500 dark:hover:bg-zinc-900/40 bg-orange-100 dark:bg-black/40 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between">
            <button
                on:click=move |_| {
                    let win = window();
                    win.open_with_url_and_target(link, "_blank").unwrap();
                }
                class="all-[unset] w-full cursor-pointer text-left flex items-stretch flex-col h-full justify-between "
            >
                <div class="flex flex-col justify-between gap-y-2">
                    <Show
                        when=move || { brand_as_letter }
                        fallback={
                            let color = current_color.clone();
                            let src = brand_src.clone();
                            move || {
                                view! {
                                    <img
                                        src=src.clone()
                                        loading="lazy"
                                        width="60"
                                        class=format!("rounded-full h-[60px] w-[60px] {color}")
                                        alt=name.join(" ")
                                    />
                                }
                            }
                        }
                    >
                        <span class=format!(
                            "h-[60px] w-[60px] rounded-full text-4xl flex justify-center items-center {current_color}",
                        )>{brand_src.clone()}</span>
                    </Show>
                    <CardTitle texts=name />
                    <p class="mt-2 font-work-sans text-black dark:text-white">{description}</p>
                </div>
                <div class="flex gap-4 sm:gap-0 justify-around items-center mt-4">
                    <ButtonLink href=button_link size="tiny">
                        <Show
                            when=move || { !button_text.is_empty() }
                            fallback=move || name.join(" ")
                        >
                            <span>{button_text}</span>
                        </Show>
                    </ButtonLink>
                    <span class="px-1">
                        <GithubIcon size=30 />
                    </span>
                </div>
            </button>
        </div>
    }
}
