use leptos::*;

use crate::components::{
    community_projects::CommunityProjects, footer::Footer, hero::Hero,
    other_communities::OtherCommunities, our_communities::OurCommunities,
};

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div>
            <Hero />
            <main>
                <OurCommunities />
                <OtherCommunities />
                <CommunityProjects />
            </main>
            <Footer />
        </div>
    }
}
