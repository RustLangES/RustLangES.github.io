use leptos::{component, view, IntoView};

use crate::components::{CommunityProjects, Hero, OtherCommunities, OurCommunities, Hacktoberfest, Sponsors};

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div>
            <Hero/>
            <OurCommunities/>
            <Hacktoberfest/>
            <CommunityProjects show_more=true/>
            <OtherCommunities show_more=true/>
            <Sponsors/>
        </div>
    }
}
