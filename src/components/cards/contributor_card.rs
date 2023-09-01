use leptos::*;

use crate::components::icons::github_icon::GithubIcon;

#[component]
pub fn ContributorCard(
    #[prop(into)] name: String,
    #[prop(into)] description: String,
    #[prop(into)] link: String,
    #[prop(into)] brand_src: String,
) -> impl IntoView {
    view! {
        <article>
            <a
                href=link
                target="_blank"
                class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between h-full"
            >
                <div class="flex flex-col gap-y-2">
                    <img src=brand_src width="60" class="rounded-full mb-4" alt=name.clone() />
                    <h2 class="font-work-sans text-black text-xl">{name.clone()}</h2>
                    <p class="font-work-sans text-black">{description}</p>
                </div>
                <span class="ml-auto">
                    <GithubIcon size=30/>
                </span>
            </a>
        </article>
    }
}
