use leptos::*;

use crate::components::hero::Hero;
use crate::components::our_communities::OurCommunities;
use crate::components::other_communities::OtherCommunities;

#[component]
pub fn Index() -> impl IntoView {
    view!{ 
        <div>
            <Hero />
            <main>
                <OurCommunities />
                <OtherCommunities />
            </main>
        </div>
    }
}
