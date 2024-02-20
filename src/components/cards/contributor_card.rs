use leptos::{IntoView, component, view};

use crate::components::icons::{GithubIcon, TwitterIcon, LocationIcon};

#[component]
pub fn ContributorCard(
    #[prop(into)] name: String,
    #[prop(into)] description: Option<String>,
    #[prop(into)] twitter: Option<String>,
    #[prop(into)] location: Option<String>,
    #[prop(into)] link: String,
    #[prop(into)] brand_src: String,
    #[prop(into)] contributions: i32,
) -> impl IntoView {

    view! {
        <article>
            <a
                href=link.clone()
                target="_blank"
                class="group flex flex-col gap-y-6 border border-black p-4 hover:bg-orange-500 bg-orange-100 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between h-full"
            >
                <span class="absolute top-0 end-0 inline-flex items-center size-3.5 group-hover:min-w-16 rounded-full border-2 border-white text-xs font-medium transition-all transform -translate-y-1/2 translate-x-1/2 bg-teal-500 dark:border-slate-900 badge-container">
                    <span class="sr-only text-black badge-content transition-all transform">{contributions}</span>
                </span>
                <div class="flex flex-col gap-y-2">
                    <img src=brand_src width="60" class="rounded-full mb-4" alt=name.clone()/>
                    <h2 class="font-work-sans text-black text-xl">{name}</h2>
                    {location
                        .map_or_else(
                            || view! { <div class="hidden"></div> },
                            |location| {
                                view! {
                                    <div class="flex gap-2 items-center bg-slate-200/20 rounded-md p-1">
                                        <LocationIcon size=16/>
                                        <p class="font-work-sans text-black text-sm">{location}</p>
                                    </div>
                                }
                            },
                        )}

                    <p class="font-work-sans text-black">{description}</p>
                </div>
                <span class="ml-auto flex">
                    {twitter
                        .map_or_else(
                            || view! { <div></div> },
                            |twitter| {
                                view! {
                                    <div>
                                        <a
                                            href=format!("https://twitter.com/{}", twitter)
                                            target="_blank"
                                        >
                                            <TwitterIcon size=30/>
                                        </a>
                                    </div>
                                }
                            },
                        )}
                    <a href=link target="_blank">
                        <GithubIcon size=30/>
                    </a>
                </span>
            </a>
        </article>
    }
}
