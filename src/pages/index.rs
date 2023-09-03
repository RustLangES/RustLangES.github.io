use leptos::*;

use crate::components::{
    community_projects::CommunityProjects, hero::Hero, other_communities::OtherCommunities,
    our_communities::OurCommunities,
};

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div>
            <Hero/>
            <OurCommunities/>
            <OtherCommunities/>
            <CommunityProjects/>
        </div>
    }
}
