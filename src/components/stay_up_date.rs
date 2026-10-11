use leptos::prelude::*;
use rustlanges_components::{
    button::{Button, Variant as ButtonVariant},
    card::Card,
};

#[component]
pub fn StayUpDate() -> impl IntoView {
    view! {
        <div class="w-full bg-primary-500 flex flex-col items-center justify-center overflow-clip text-black">
            <div class="w-full container max-w-7xl flex flex-col-reverse md:flex-row items-center justify-between m-auto p-6">
                <div class="flex flex-col justify-center gap-8">
                    <h2 class="text-heading-2 font-heading-2 text-4xl font-bold">
                        "Mantente al día"
                    </h2>
                    <p class="font-heading-3 text-xl">
                        Entérate de todos los
                        <span class="font-bold">eventos y actividades en linea</span>
                        en los que puedes participar en nuestro Discord
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
