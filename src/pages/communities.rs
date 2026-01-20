use leptos::{component, view, IntoView};

use crate::components::OtherCommunities;

#[component]
pub fn Communities() -> impl IntoView {
    view! {
        <div>
            <OtherCommunities />
            <OtherCommunities other_communities=true />
        </div>
    }
}
