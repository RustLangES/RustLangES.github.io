use leptos::{component, prelude::*, view, IntoView};

use crate::components::CommunityProjects;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="mx-auto">
            <CommunityProjects />

        // TODO: Fetch all our projects from Discord using our API
        </div>
    }
}
