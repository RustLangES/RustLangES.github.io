use leptos::*;
use crate::components::hero::Hero;
use crate::components::layout::other_communities::OtherCommunities;

#[component]
pub fn Index(
    cx: Scope
) -> impl IntoView {
    view!{ cx,
        <div>
            <Hero />
            <div class="bg-grid-slate-300 dark:bg-grid-slate-900">
                <OtherCommunities />
            </div>
        </div>
    }
}
