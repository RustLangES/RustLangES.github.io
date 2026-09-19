use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant as ButtonVariant},
    card::Card,
    chip::{Chip, Variant as ChipVariant},
    icons::{Github, Twitter},
};

struct Contributor {
    avatar: String,
    name: String,
    location: String,
}

#[allow(non_snake_case)]
pub fn Contributors() -> impl IntoView {
    let sergio = Contributor { name: "Sergio Ribera".to_string(), avatar: "https://avatars.githubusercontent.com/u/56278796?u=9e3dac947b4fd3ca2f1a05024e083c64e4c69cfe&v=4".to_string(), location: "Ubicación".to_string() };
    let juanperias = Contributor { name: "Juanperias".to_string(), avatar: "https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4".to_string(), location: "Ubicación".to_string() };
    let phosphorus = Contributor {
        name: "Phosphorus-M".to_string(),
        avatar: "https://avatars.githubusercontent.com/u/19656993?s=400&v=4".to_string(),
        location: "Ubicación".to_string(),
    };

    let contributors = vec![sergio, juanperias, phosphorus];

    let contributors_list = contributors
        .into_iter()
        .map(|c| {
            view! {
                <div class="min-w-87.5 max-w-md h-full mt-5">
                    <Card class="min-w-87.5 max-w-md h-full">
                        <div class="flex flex-col gap-4 justify-between h-full">
                            <div class="flex flex-row items-center gap-4 h-full justify-stretch">
                                <Avatar url=c.avatar.clone() size=52 class="min-w-13" />
                                <div class="w-full">
                                    <h2 class="text-heading-3 mt-2 mb-2">{{ c.name.clone() }}</h2>
                                    <Chip label=c.location.clone() variant=ChipVariant::Location />
                                </div>
                                <div class="flex gap-3 items-center justify-center">
                                    <Button
                                        variant=ButtonVariant::Icon
                                        icon=view! { <Twitter /> }.into_any()
                                        on_click=move |_| {}
                                    />
                                    <Button
                                        variant=ButtonVariant::Icon
                                        icon=view! { <Github /> }.into_any()
                                        on_click=move |_| {}
                                    />
                                </div>
                            </div>
                        </div>
                    </Card>
                </div>
            }
        })
        .collect_view();

    view! {
        <section class="bg-light dark:bg-dark p-4 lg:p-20 w-full">
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
                <Button
                    variant=ButtonVariant::Secondary
                    label="Conócelos a todos"
                    on_click=move |_| {}
                />
            </div>
        </section>
    }
}
