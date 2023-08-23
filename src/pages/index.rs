use leptos::*;
use crate::components::layout::hero::Hero;
use crate::components::layout::community_links::CommunityLinks;

#[component]
pub fn Index(
    cx: Scope
) -> impl IntoView {
    view!{ cx,
        <div class=" bg-grid-slate-300 dark:bg-grid-slate-400">
            <Hero />
            <CommunityLinks />
        </div>
    }
}