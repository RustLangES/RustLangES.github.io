use leptos::*;

use crate::components::{CommunityProjects, Hero, OtherCommunities, OurCommunities};

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div>
            <Hero/>
            <OurCommunities/>
            <CommunityProjects main={true}/>
            <OtherCommunities main={true}/>
        </div>
    }
}
