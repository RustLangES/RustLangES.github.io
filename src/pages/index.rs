use leptos::*;

use crate::components::hero::Hero;
use crate::components::footer::Footer;
use crate::components::our_communities::OurCommunities;
use crate::components::other_communities::OtherCommunities;
use crate::components::community_projects::CommunityProjects;

#[component]
pub fn Index() -> impl IntoView {
    view!{
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
