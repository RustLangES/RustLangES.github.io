use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant as ButtonVariant},
};

use crate::{
    components::{community_project::CommunityProjectSection, why_rust::WhyRust},
    models::GithubUser,
};

#[component]
pub fn Communities() -> impl IntoView {
    let juanperas = GithubUser{ username: "juanperas", avatar: "https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4" };
    let apika = GithubUser { username: "ApikaLucas", avatar: "https://avatars.githubusercontent.com/u/70247585?u=513513290efb6dc162afc899646e8e0467cedfc2&v=4"};
    let sergio = GithubUser { username: "Sergio", avatar: "https://avatars.githubusercontent.com/u/56278796?u=9e3dac947b4fd3ca2f1a05024e083c64e4c69cfe&v=4"};

    let users = vec![
        juanperas.clone(),
        apika.clone(),
        sergio.clone(),
        juanperas.clone(),
    ];

    let avatars = users
        .iter()
        .skip(1)
        .map(|user| {
            view! { <Avatar class="-ml-4 border-black" url=user.avatar alt=user.username size=64 /> }
        })
        .collect_view();

    view! {
        <section>
            <div class="w-full h-[65dvh] rustlang-es-background dark:bg-[#F04906]  text-akira flex items-center justify-center">
                <div class="container max-w-7xl  flex  flex-col-reverse md:flex-row items-center justify-center m-auto">
                    <div class="flex flex-col justify-center gap-8 mx-auto">
                        <div class="flex flex-col gap-2 justify-center not-md:items-center w-full text-center lg:text-left">
                            <p class="uppercase text-h1 font-heading-1 text-akira font-bold text-primary-500 dark:text-primary-200">
                                La comunidad
                            </p>
                            <p class="uppercase text-h1 font-heading-1 text-akira font-bold">
                                de Rust en español
                            </p>
                            <p class="text-heading-4 font-heading-4 max-w-md">
                                Un espacio donde <span class="font-bold">aprender</span>,
                                <span class="font-bold">compartir</span>y
                                <span class="font-bold">potenciar</span>
                                tus habilidades en el lenguaje.
                            </p>
                        </div>
                        <div class="flex gap-4 flex-wrap w-full not-md:justify-center not-md:items-center">
                            <Button
                                variant=ButtonVariant::Primary
                                class="bg-light font-body"
                                on_click=|_| {}
                                label="¡Únete a la Comunidad!"
                            />
                        </div>
                    </div>
                    <div>
                        <img
                            src="/assets/new/images/comunidad-hero.png"
                            alt=""
                            class="max-w-3xs lg:max-w-md"
                        />
                        <div class="flex flex-row justify-center items-center gap-8 mt-8">
                            <div>
                                <h3 class="text-primary-500 dark:text-primary-200 text-heading-1">
                                    +1000
                                </h3>
                                <h3 class="text-primary-500 dark:text-primary-200 text-body font-body text-right">
                                    Rustáceos
                                </h3>
                            </div>
                            <div class="flex items-center justify-center ">
                                <Avatar
                                    class="border-black"
                                    url=users[0].avatar
                                    alt=users[0].username
                                    size=64
                                />
                                {avatars}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
        <WhyRust />
        <CommunityProjectSection many_projects=true />
    }
}
