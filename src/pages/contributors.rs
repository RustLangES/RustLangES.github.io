use leptos::prelude::*;

use crate::{
    components::{
        contributor_card::ContributorCard,
        footer::Footer,
        ranking::{
            Ranking,
            RankingBrick::{self, *},
        },
    },
    extras::CONTRIBUTORS,
};

const DESKTOP_BRICKS: &[RankingBrick] = &[
    Place(1, "col-start-11"),
    Place(2, "col-start-6"),
    Empty("col-start-10"),
    Empty("col-start-7"),
    Empty("col-start-11"),
    Place(3, "col-start-15"),
    Place(4, "col-start-1"),
    Empty("col-start-5"),
    Empty("col-start-9"),
    Empty("col-start-13"),
    Empty("col-start-3"),
    Empty("col-start-7"),
    Empty("col-start-11"),
    Empty("col-start-15"),
    Place(5, "col-start-19"),
];

const MOBILE_BRICKS: &[RankingBrick] = &[
    Place(1, "col-start-6"),
    Place(2, "col-start-1"),
    Empty("col-start-5"),
    Empty("col-start-2"),
    Empty("col-start-6"),
    Place(3, "col-start-10"),
];

const BRICK_COLORS: &[&str] = &[
    "bg-primary-500",
    "bg-secondary-500",
    "bg-primary-300",
    "bg-secondary-300",
    "bg-neutral-50 dark:bg-neutral-900",
    "bg-primary-200",
    "bg-secondary-400",
    "bg-primary-400",
    "bg-primary-100",
];

#[component]
pub fn Contributors() -> impl IntoView {
    let contributors_list = CONTRIBUTORS
        .iter()
        .map(|c| view! { <ContributorCard contributor=c /> })
        .collect_view();

    view! {
        <div class="w-full py-12 tablet:py-16 rustlang-es-background dark:bg-[#F04906] flex items-center justify-center">
            <h1 class="uppercase text-akira heading text-[22px] tablet:text-[28px] desktop:text-[40px] text-primary-500 dark:text-light">
                "Colaboradores"
            </h1>
        </div>
        <section class="bg-light dark:bg-dark w-full px-4 py-12 desktop:py-16">
            <div class="flex flex-col items-center gap-10 w-full max-w-[1046px] mx-auto">
                <div class="flex flex-col items-center gap-4 text-center">
                    <h2 class="text-h2">"Ranking de contribuciones"</h2>
                    <p class="text-subtitle-1 max-w-lg">
                        "Su pasión por el código abierto y el desarrollo de Rust es el corazón que impulsa nuestro crecimiento."
                    </p>

                </div>
                <Ranking desktop=DESKTOP_BRICKS mobile=MOBILE_BRICKS colors=BRICK_COLORS />
                <div class="grid grid grid-cols-1 tablet:grid-cols-2 xl:grid-cols-3 gap-6 w-full">
                    {contributors_list}
                </div>
            </div>
        </section>
        <Footer />
    }
}
