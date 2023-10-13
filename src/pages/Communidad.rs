use leptos::*;

use crate::components::{community_projects::CommunityProjects, other_communities::OtherCommunities};

#[component]
pub fn Communidad() -> impl IntoView {
    view! {
        <div>
            <CommunityProjects/>
            <OtherCommunities/>
        </div>
    }
}
