use leptos::prelude::*;
use rustlanges_components::button::{Button, Variant as ButtonVariant};

use crate::{components::contributor_card::ContributorCard, extras::CONTRIBUTORS};

#[allow(non_snake_case)]
pub fn Contributors() -> impl IntoView {
    let contributors_list = CONTRIBUTORS
        .iter()
        .take(6)
        .map(|c| {
            view! {
                <div class="min-w-87.5 max-w-md h-full mt-5">
                    <ContributorCard contributor=c class="min-w-87.5 max-w-md" />
                </div>
            }
        })
        .collect_view();

    view! {
        <section class="bg-light dark:bg-dark dark:bg-dark p-4 lg:p-20 w-full">
            <div class="flex flex-col justify-center items-center w-full container xl:max-w-7xl lg:px-36 lg:pb-20 mx-auto">
                <h2 class="text-h2 text-center mb-4">"Colaboradores"</h2>
                <div>
                    <p class="text-center max-w-lg">
                        "Gracias a su esfuerzo, los servicios de nuestra comunidad"
                    </p>
                    <p class="text-center max-w-lg">
                        "se mantienen activos y en constante evolución."
                    </p>
                </div>
            </div>

            <div class="flex flex-row justify-center items-center gap-4 max-w-full m-auto">
                <div class="items-center gap-8 overflow-x-auto p-4 xl:grid xl:grid-cols-3">
                    {contributors_list}
                </div>
            </div>

            <div class="flex flex-row justify-center items-center m-auto w-full mt-10">
                <a href="/colaboradores">
                    <Button
                        variant=ButtonVariant::Secondary
                        label="Conócelos a todos"
                        on_click=move |_| {}
                    />
                </a>
            </div>
        </section>
    }
}
