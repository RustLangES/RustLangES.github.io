use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant as ButtonVariant},
    card::Card,
};

use crate::{
    components::{
        community_project::CommunityProjectSection,
        contributor_card::{ContributorCard, ContributorInfo},
        footer::Footer,
        why_rust::WhyRust,
    },
    models::GithubUser,
};

struct OtherCommunity {
    name: &'static str,
    flag: &'static str,
    description: &'static str,
    link: &'static str,
}

#[component]
pub fn Communities() -> impl IntoView {
    let juanperas = GithubUser {
        username: "juanperas",
        avatar: "https://avatars.githubusercontent.com/u/136520331?u=7353ba372f09091049692d6e95f5a8cd8a42565f&v=4",
    };
    let apika = GithubUser {
        username: "ApikaLucas",
        avatar: "https://avatars.githubusercontent.com/u/70247585?u=513513290efb6dc162afc899646e8e0467cedfc2&v=4",
    };
    let sergio = GithubUser {
        username: "Sergio",
        avatar: "https://avatars.githubusercontent.com/u/56278796?u=9e3dac947b4fd3ca2f1a05024e083c64e4c69cfe&v=4",
    };

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

    let contributors_preview = vec![
        ContributorInfo {
            login: "sergioribera".to_string(),
            avatar_url: "https://avatars.githubusercontent.com/u/56278796?v=4".to_string(),
            url: "https://github.com/sergioribera".to_string(),
            twitter_username: None,
            location: Some("Bolivia".to_string()),
            total_contributions: 1250,
        },
        ContributorInfo {
            login: "ApikaLucas".to_string(),
            avatar_url: "https://avatars.githubusercontent.com/u/70247585?v=4".to_string(),
            url: "https://github.com/ApikaLucas".to_string(),
            twitter_username: None,
            location: None,
            total_contributions: 430,
        },
        ContributorInfo {
            login: "juanperas".to_string(),
            avatar_url: "https://avatars.githubusercontent.com/u/136520331?v=4".to_string(),
            url: "https://github.com/juanperas".to_string(),
            twitter_username: None,
            location: None,
            total_contributions: 215,
        },
    ];

    let other_communities: Vec<OtherCommunity> = vec![
        OtherCommunity {
            name: "Rust Colombia",
            flag: "🇨🇴",
            description: "Comunidad oficial de Rust en Colombia.",
            link: "#",
        },
        OtherCommunity {
            name: "Rust Venezuela",
            flag: "🇻🇪",
            description: "Comunidad de Rust en Venezuela.",
            link: "#",
        },
        OtherCommunity {
            name: "Rust Argentina",
            flag: "🇦🇷",
            description: "Comunidad de Rust en Argentina.",
            link: "#",
        },
        OtherCommunity {
            name: "Rust Ecuador",
            flag: "🇪🇨",
            description: "Comunidad de Rust en Ecuador.",
            link: "#",
        },
        OtherCommunity {
            name: "Rust México",
            flag: "🇲🇽",
            description: "Comunidad de Rust en México.",
            link: "#",
        },
        OtherCommunity {
            name: "Rust Perú",
            flag: "🇵🇪",
            description: "Comunidad de Rust en Perú.",
            link: "#",
        },
    ];

    view! {
        // Hero
        <div class="w-full min-h-[65dvh] rustlang-es-background dark:bg-[#F04906] text-akira flex items-center justify-center py-16 px-6">
            <div class="container max-w-7xl flex flex-col-reverse md:flex-row items-center justify-center gap-8 lg:gap-16 m-auto">
                <div class="flex flex-col justify-center gap-8 mx-auto lg:mx-0 text-center lg:text-left">
                    <div class="flex flex-col gap-2">
                        <h1 class="uppercase leading-tight text-primary-500 dark:text-orange-300">
                            "La comunidad"
                        </h1>
                        <h1 class="uppercase leading-tight">
                            "de Rust en español"
                        </h1>
                        <p class="text-base font-normal font-body mt-2 leading-relaxed max-w-md">
                            "Un espacio donde "
                            <span class="font-bold">"aprender"</span>
                            ", "
                            <span class="font-bold">"compartir"</span>
                            " y "
                            <span class="font-bold">"potenciar"</span>
                            " tus habilidades en el lenguaje."
                        </p>
                    </div>
                    <div class="flex gap-4 flex-wrap justify-center lg:justify-start">
                        <Button
                            variant=ButtonVariant::Primary
                            on_click=|_| {}
                            label="¡Únete a la Comunidad!"
                        />
                    </div>
                </div>
                <div class="flex-shrink-0 flex flex-col items-center gap-6">
                    <img
                        src="/assets/new/images/comunidad-hero.png"
                        alt="Comunidad RustLangES"
                        class="w-40 lg:w-56 xl:w-72"
                    />
                    <div class="flex flex-row justify-center items-center gap-8">
                        <div>
                            <h3 class="text-primary-500 dark:text-orange-300 text-4xl font-bold">"+1000"</h3>
                            <p class="text-primary-500 dark:text-orange-300 text-sm font-body text-right">"Rustáceos"</p>
                        </div>
                        <div class="flex items-center justify-center">
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

        <WhyRust />
        <CommunityProjectSection many_projects=true />

        // Colaboradores preview
        <section class="bg-white dark:bg-neutral-900 py-20 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col items-center gap-8">
                <div class="flex flex-col items-center gap-3">
                    <h2 class="text-h2 text-center">"Colaboradores"</h2>
                    <p class="text-center max-w-lg leading-relaxed">
                        "Personas que contribuyen con código, traducciones y contenido a nuestros proyectos."
                    </p>
                </div>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 w-full max-w-3xl">
                    {contributors_preview.into_iter().map(|c| view! {
                        <ContributorCard contributor=c />
                    }).collect_view()}
                </div>
                <Button
                    variant=ButtonVariant::Primary
                    label="Conócelos a todos"
                    on_click=|_| {}
                />
            </div>
        </section>

        // Otras Comunidades Rust
        <section class="bg-orange-50 dark:bg-neutral-950 py-20 w-full">
            <div class="container max-w-7xl mx-auto px-6 flex flex-col items-center gap-8">
                <div class="flex flex-col items-center gap-3">
                    <h2 class="text-h2 text-center">"Otras Comunidades Rust"</h2>
                    <p class="text-center max-w-lg">
                        "Conoce otras comunidades de Rust en habla hispana."
                    </p>
                </div>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 w-full">
                    {other_communities.into_iter().map(|comm| view! {
                        <Card class="flex flex-col items-center gap-4 p-6 text-center">
                            <span class="text-5xl">{comm.flag}</span>
                            <h3 class="font-bold text-lg">{comm.name}</h3>
                            <p class="text-sm text-neutral-600 dark:text-neutral-400 flex-1">
                                {comm.description}
                            </p>
                            <a href=comm.link target="_blank" class="w-full">
                                <Button
                                    variant=ButtonVariant::Secondary
                                    label="Discord"
                                    class="w-full"
                                    on_click=|_| {}
                                />
                            </a>
                        </Card>
                    }).collect_view()}
                </div>
            </div>
        </section>

        <Footer />
    }
}
