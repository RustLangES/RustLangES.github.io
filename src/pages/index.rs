use leptos::*;

use crate::components::{CommunityProjects, Hero, OtherCommunities, OurCommunities};

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div>
            <Hero/>
            <OurCommunities/>
            <CommunityProjects show_more=true/>
            <OtherCommunities show_more=true/>
        </div>
    }
}