use leptos::{component, view, IntoView};

use crate::components::icons::{GithubIcon, LocationIcon, TwitterIcon};

#[component]
pub fn ContributorCard(
    #[prop(into)] name: String,
    #[prop(into)] description: Option<String>,
    #[prop(into)] twitter: Option<String>,
    #[prop(into)] location: Option<String>,
    #[prop(into)] link: String,
    #[prop(into)] brand_src: String,
    #[prop(into)] contributions: u64,
) -> impl IntoView {
    view! {
        <article class="hover:z-10 flex flex-col h-full gap-y-6 border border-black p-4 hover:bg-orange-500 bg-orange-100 dark:hover:bg-zinc-900/40 dark:bg-black/40 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between">
            <a href=link.clone() target="_blank" class="group flex flex-col justify-between">
                <span class="absolute top-0 end-0 inline-flex items-center size-3.5 group-hover:min-w-16 rounded-full border-2 border-white text-xs font-medium transition-all transform -translate-y-1/2 translate-x-1/2 bg-teal-500 dark:border-slate-900 badge-container">
                    <span class="sr-only text-black badge-content transition-all transform">
                        {contributions}
                    </span>
                </span>
                <div class="flex flex-col gap-y-2">
                    <img src=brand_src width="60" class="rounded-full mb-4" alt=name.clone() />
                    <h2 class="font-work-sans text-black dark:text-white text-xl">{name}</h2>
                    {location
                        .map(|location| {
                            view! {
                                <div class="flex gap-2 items-center bg-slate-200/20 dark:bg-neutral-500/40 rounded-md p-1">
                                    <LocationIcon size=16 />
                                    <p class="font-work-sans text-black dark:text-white text-sm">
                                        {location}
                                    </p>
                                </div>
                            }
                        })}

                    <p class="font-work-sans text-black dark:text-white">{description}</p>
                </div>
            </a>
            <div class="ml-auto flex flex-row gap-2">
                {twitter
                    .map(|twitter| {
                        view! {
                            <a href=format!("https://twitter.com/{}", twitter) target="_blank">
                                <TwitterIcon size=30 />
                            </a>
                        }
                    })} <a href=link target="_blank">
                    <GithubIcon size=30 />
                </a>
            </div>
        </article>
    }
}
