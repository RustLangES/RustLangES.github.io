use leptos::prelude::*;
use rustlanges_components::button::{Button, Variant as ButtonVariant};

#[component]
pub fn BecameSponsorSection() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center w-full gap-8 py-12 md:py-20 rustlang-es-background dark:bg-neutral-950">
            <div class="container px-4 flex flex-col justify-center items-center gap-6">
                <h2 class="text-heading-2 md:text-heading-1 font-bold text-center">
                    Conviértete en
                    <span class="text-secondary-700 dark:text-primary-500">Sponsor</span>
                </h2>
                <p class="text-center text-paragraph-1 mb-2 md:mb-8 max-w-xl">
                    "Queremos que nadie se vea limitado en el acceso al conocimiento por razones económicas. "
                    <span class="font-bold">
                        Con tu apoyo puedes ayudarnos a lograr que nuestras acciones lleguen a todas las personas interesadas en Rust
                    </span>
                </p>
                <Button variant=ButtonVariant::Secondary on_click=|_| {} label="Ser Sponsor" />
            </div>
        </section>
    }
}
