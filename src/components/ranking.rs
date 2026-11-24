use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    card::Card,
    chip::{Chip, Variant as ChipVariant},
    icons::StarBold,
};

use crate::{extras::CONTRIBUTORS, models::Contributor};

pub enum RankingBrick {
    Place(usize, &'static str),
    Empty(&'static str),
}

use RankingBrick::*;

#[component]
pub fn Ranking(
    desktop: &'static [RankingBrick],
    mobile: &'static [RankingBrick],
    colors: &'static [&'static str],
) -> impl IntoView {
    (!CONTRIBUTORS.is_empty()).then(|| {
        view! {
            <div class="hidden xl:block w-full pt-24">
                <RankingGrid bricks=desktop colors=colors vertical=false />
            </div>
            <div class="xl:hidden w-full max-w-md pt-36">
                <RankingGrid bricks=mobile colors=colors vertical=true />
            </div>
        }
    })
}

#[component]
fn RankingGrid(
    bricks: &'static [RankingBrick],
    colors: &'static [&'static str],
    vertical: bool,
) -> impl IntoView {
    let bricks = bricks
        .iter()
        .zip(colors.iter().cycle())
        .map(|(brick, color)| {
            let (position, contributor) = match brick {
                Place(place, position) => (position, CONTRIBUTORS.get(place - 1)),
                Empty(position) => (position, None),
            };
            view! { <Brick position=position color=color contributor=contributor vertical=vertical /> }
        })
        .collect_view();

    view! { <div class="grid auto-cols-fr isolate w-full">{bricks}</div> }
}

#[component]
fn Brick(
    position: &'static str,
    color: &'static str,
    contributor: Option<&'static Contributor>,
    vertical: bool,
) -> impl IntoView {
    let height = if vertical { "h-8" } else { "h-14" };
    let studs = (0..4)
        .map(|_| view! { <span class=format!("w-1/6 h-2 border-2 border-black {color}") /> })
        .collect_view();
    let card = contributor.map(|contributor| {
        view! {
            <a
                href=contributor.github_url()
                target="_blank"
                rel="noopener noreferrer"
                class="absolute bottom-full inset-x-0 mb-4 z-10"
            >
                <RankingCard contributor=contributor vertical=vertical />
            </a>
        }
    });

    view! {
        <div class=format!(
            "relative col-span-4 {height} -mt-0.5 -ml-0.5 border-2 border-black shadow-[4px_4px_0_#000] {color} {position}",
        )>
            <div class="absolute -top-2 inset-x-0 flex justify-evenly -z-10">{studs}</div>
            {card}
        </div>
    }
}

#[component]
fn RankingCard(contributor: &'static Contributor, vertical: bool) -> impl IntoView {
    let name = contributor.display_name();
    let (layout, align) = if vertical {
        ("flex-col text-center", "items-center")
    } else {
        ("flex-row", "items-start")
    };

    view! {
        <Card clickable=true class=format!("flex {layout} items-center gap-2 p-3 w-full")>
            <div class="relative shrink-0">
                <Avatar url=contributor.avatar alt=name size=52 />
                <span class="absolute -top-1 -right-1 grid place-items-center size-5 rounded-full border border-black bg-primary-500 text-black">
                    <StarBold size=12 />
                </span>
            </div>
            <div class=format!("flex flex-col {align} gap-1 min-w-0 w-full")>
                <p class="text-paragraph-2 truncate w-full" title=name>
                    {name}
                </p>
                <Chip
                    label=contributor.contributions.to_string()
                    numeric_symbol=""
                    variant=ChipVariant::Numeric
                    class="min-h-0 py-0 px-3 text-caption"
                />
            </div>
        </Card>
    }
}
