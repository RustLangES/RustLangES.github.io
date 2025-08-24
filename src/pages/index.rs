use leptos::prelude::*;
use rustlanges_components::badge::{Type, Variant};


#[component]
pub fn Index() -> impl IntoView {
    let (gcount , wcount) = signal(20);
    view! {
        <div>
            <rustlanges_components::badge::Badge count=gcount variant=Variant::Reading r#type=Type::Text/>
        </div>
    }
}
