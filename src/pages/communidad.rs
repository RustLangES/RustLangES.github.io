use leptos::{IntoView, component, tracing, view};

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