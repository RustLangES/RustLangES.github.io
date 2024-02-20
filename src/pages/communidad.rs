use leptos::{component, view, IntoView};

use crate::components::{CommunityProjects, OtherCommunities};

#[component]
pub fn Communidad() -> impl IntoView {
    view! {
        <div>
            <CommunityProjects/>
            <OtherCommunities/>
        </div>
    }
}
