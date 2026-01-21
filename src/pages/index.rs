use leptos::{component, view, IntoView};
use leptos::prelude::*;

use crate::components::{CommunityProjects, Hero, OtherCommunities, OurCommunities, Sponsors};

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div>
            <Hero />
            <OurCommunities />
            <CommunityProjects show_more=true />
            <OtherCommunities show_more=true />
            <Sponsors />
        </div>
    }
}
