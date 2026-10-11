use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant as ButtonVariant},
    card::Card,
};

#[component]
pub fn Agenda() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col items-center justify-center">
            <div class="w-full container max-w-7xl flex flex-col-reverse md:flex-row items-center justify-between m-auto p-6">
                <div class="flex flex-col justify-center gap-8">
                    <h2 class="text-heading-2 font-heading-2 text-4xl font-bold">"Agenda 2026"</h2>
                    <p class="font-heading-3 text-xl">
                        Revisa nuestro calendario y anticípate a reservar tu lugar
                    </p>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=|_| {}
                        class="text-heading-4 font-heading-4 font-semibold"
                        label="¡Únete!"
                    />
                </div>
                <div>
                    <img src="/assets/stay_up_date.png" />
                </div>
            </div>
        </div>
    }
}
