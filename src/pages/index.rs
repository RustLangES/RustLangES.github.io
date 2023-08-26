use leptos::*;

use crate::components::hero::Hero;
use crate::components::our_communities::OurCommunities;
use crate::components::other_communities::OtherCommunities;

#[component]
pub fn Index(
    cx: Scope
) -> impl IntoView {
    view!{ cx,
        <div>
            <Hero />
            <main>
                <OurCommunities />
                <OtherCommunities />
            </main>
        </div>
    }
}
