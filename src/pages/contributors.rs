use leptos::*;

use crate::components::{cards::contributor_card::ContributorCard, footer::Footer, header::Header};

struct ContributorItem {
    name: &'static str,
    description: &'static str,
    link: &'static str,
    brand_src: &'static str,
}

#[component]
pub fn Contributors() -> impl IntoView {
    let contributors: Vec<ContributorItem> = vec![
        ContributorItem {
            name: "Phosphorus Moscu",
            description: "Student in Computer Science degree at Universidad Nacional del Oeste | Developer Consultant at Globant",
            link: "https://github.com/Phosphorus-M",
            brand_src: "https://avatars.githubusercontent.com/u/19656993?v=4",
        },
        ContributorItem {
            name: "Sergio Alejandro Ribera Costa",
            description: "Enthusiastic developer Linux and Open Source lover",
            link: "https://github.com/SergioRibera",
            brand_src: "https://avatars.githubusercontent.com/u/56278796?v=4",
        },
        ContributorItem {
            name: "Michael Cardoza",
            description: "Software Developer",
            link: "https://github.com/michaelcardoza",
            brand_src: "https://avatars.githubusercontent.com/u/8800455?v=4",
        },
        ContributorItem {
            name: "Emilio Ruscitti",
            description: "Rust Dev",
            link: "https://github.com/Lemin-n",
            brand_src: "https://avatars.githubusercontent.com/u/88170949?v=4",
        },
        ContributorItem {
            name: "carbon based lifeform",
            description: "Rust Dev",
            link: "https://github.com/ph4un00b",
            brand_src: "https://avatars.githubusercontent.com/u/1057021?v=4",
        },
        ContributorItem {
            name: "CrawKatt",
            description: "Aprendiz de Rust, estudiante de AIEP, Chileno",
            link: "https://github.com/CrawKatt",
            brand_src: "https://avatars.githubusercontent.com/u/108593932?v=4",
        },
    ];

    view! {
        <div>
            <Header/>
            <main>
                <section class="bg-orange-300/30 py-16">
                    <div class="flex flex-col gap-y-6 container mx-auto px-4">
                        <h2 class="text-3xl text-left mb-6">
                            <span class="font-work-sans font-light">"Nuestros "</span>
                            <span class="font-alfa-slab text-orange-500">"Colaboradores"</span>
                        </h2>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-5 gap-6">
                            {contributors
                                .into_iter()
                                .map(|contributor| {
                                    view! {
                                        <ContributorCard
                                            name=contributor.name
                                            description=contributor.description
                                            link=contributor.link
                                            brand_src=contributor.brand_src
                                        />
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>
                </section>
            </main>
            <Footer/>
        </div>
    }
}
