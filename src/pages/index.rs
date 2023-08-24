use leptos::*;
use crate::components::layout::hero::Hero;
use crate::components::layout::community_links::CommunityLinks;
use crate::components::layout::other_communities::OtherCommunities;

#[component]
pub fn Index(
    cx: Scope
) -> impl IntoView {
    view!{ cx,
        <div class="bg-grid-slate-300 dark:bg-grid-slate-900">
            <Hero />
            <CommunityLinks />
            <OtherCommunities />
        </div>
    }
}