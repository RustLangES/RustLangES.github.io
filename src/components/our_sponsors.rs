use leptos::{leptos_dom::logging::console_warn, prelude::*};

use crate::components::sponsor_block::{SponsorBlock, SponsorVariant};

#[derive(Default)]
struct SponsorDef {
    variant: SponsorVariant,

    img: &'static str,
    alt: &'static str,
    class: &'static str,
}

enum SponsorBrick {
    Sponsor(&'static str),
    Empty(&'static str),
    Space(&'static str),
}

fn sponsors() -> impl IntoView {
    use SponsorVariant::*;

    let mut sponsors_buf = vec![
        SponsorDef {
            variant: Black,
            img: "/assets/sponsors/sysarmy.webp",
            alt: "SysArmy",
            ..Default::default()
        },
        SponsorDef {
            variant: White,
            img: "/assets/new/sponsors/testing-bolivia.png",
            alt: "Testing Bolivia",
            class: "mix-blend-difference",
            ..Default::default()
        },
        SponsorDef {
            variant: Custom("#00c39d"),
            img: "/assets/new/sponsors/frontendcafe.png",
            alt: "FrontendCafe",
            class: "shadow-frontendcafe w-full h-full object-contain",
            ..Default::default()
        },
        SponsorDef {
            variant: Custom("#193270"),
            img: "/assets/new/sponsors/universidad_nur.png",
            alt: "Universidad Nur",
            ..Default::default()
        },
        SponsorDef {
            variant: Black,
            img: "/assets/sponsors/shuttle.webp",
            alt: "ShuttleRS",
            class: "mix-blend-difference",
            ..Default::default()
        },
        SponsorDef {
            variant: Primary,
            img: "/assets/new/sponsors/cloudflare.svg",
            alt: "Cloudflare",
            ..Default::default()
        },
        SponsorDef {
            variant: Custom("#051024"),
            img: "/assets/new/sponsors/crabnebula.svg",
            alt: "Crabnebula",
            class: "mix-blend-difference",
        },
        SponsorDef {
            variant: Custom("var(--color-neutral-950)"),
            img: "/assets/new/sponsors/tauri-logo.png",
            alt: "Tauri",
            ..Default::default()
        },
        SponsorDef {
            variant: White,
            img: "/assets/new/sponsors/aws.svg",
            alt: "Amazon Web Services",
            ..Default::default()
        },
    ];

    // Sponsors will be "popped" from the buffer, so reverse it
    // to get the same order as written
    sponsors_buf.reverse();

    const SPONSOR_BRICKS: usize = 26;

    const BRICKS: [SponsorBrick; SPONSOR_BRICKS] = [
        SponsorBrick::Sponsor("col-start-2 md:col-start-4"),
        SponsorBrick::Empty(""),
        SponsorBrick::Sponsor(""),
        SponsorBrick::Space("hidden md:block lg:block"),
        SponsorBrick::Space("hidden md:block lg:hidden"),
        SponsorBrick::Sponsor(""),
        SponsorBrick::Empty("lg:col-start-2"),
        SponsorBrick::Space("block sm:hidden md:hidden lg:block"),
        SponsorBrick::Sponsor("col-start-3 sm:col-start-[unset]"),
        SponsorBrick::Space("hidden md:block lg:hidden"),
        SponsorBrick::Empty("md:col-start-3 lg:col-start-[unset]"),
        SponsorBrick::Sponsor("sm:col-start-3 md:col-start-[unset]"),
        SponsorBrick::Empty(""),
        SponsorBrick::Sponsor(""),
        SponsorBrick::Empty("col-start-4 sm:col-start-[unset]"),
        SponsorBrick::Sponsor(""),
        SponsorBrick::Empty("col-start-2 sm:col-start-[unset]"),
        SponsorBrick::Sponsor(""),
        SponsorBrick::Empty("sm:col-start-2 md:col-start-[unset]"),
        SponsorBrick::Space(""),
        SponsorBrick::Sponsor("md:col-start-2 lg:col-start-3"),
        SponsorBrick::Empty("col-start-3 md:col-start-[unset]"),
        SponsorBrick::Space("hidden md:hidden lg:block"),
        SponsorBrick::Empty(""),
        SponsorBrick::Empty("hidden sm:block"),
        SponsorBrick::Empty("hidden lg:block"),
    ];

    // Get a top-to-down zindex
    let mut next_id = SPONSOR_BRICKS + 1;
    let mut next_id = move || {
        next_id -= 1;
        next_id
    };

    let view = BRICKS.map(|b| match b {
        SponsorBrick::Sponsor(c) if let Some(sponsor) = sponsors_buf.pop() => {
            view! {
                <SponsorBlock index=next_id() variant=sponsor.variant class=c>
                    <img
                        src=sponsor.img
                        alt=sponsor.alt
                        class=format!("object-contain w-full h-full {}", sponsor.class)
                    />
                </SponsorBlock>
            }
        }
        .into_any(),
        // If there's no more sponsors, just show it as random empty brick.
        // This allows to design and assign spaces to sponsors even if not enough
        SponsorBrick::Sponsor(c) | SponsorBrick::Empty(c) => view! { <SponsorBlock index=next_id() variant=SponsorVariant::random_colored() class=c /> }
        .into_any(),
        // Reserve an entire brick.
        // Consult "What the hell???" section (below) to know more about this.
        SponsorBrick::Space(c) => view! { <div class=format!("col-span-4 {c}") /> }
        .into_any(),
    });

    if !sponsors_buf.is_empty() {
        console_warn(&format!(
            "{} Sponsors left. Some sponsors are not rendered: {}",
            sponsors_buf.len(),
            sponsors_buf
                .into_iter()
                .map(|s| s.alt)
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    view
}

#[component]
pub fn OurSponsorsSection() -> impl IntoView {
    view! {
        <section class="flex flex-col justify-center items-center w-full gap-8 py-20 dark:bg-neutral-900">
            <div class="container flex flex-col justify-center items-center gap-8">
                <h2 class="text-h2 mb-4">"Nuestros sponsors"</h2>
                <p class="text-center mb-8 max-w-lg">
                    Todos nuestros eventos y actividades son
                    <span class="font-bold">gratuitas</span>
                    gracias a las organizaciones que apoyan nuestro trabajo.
                </p>

                <div>
                    <img src="/assets/new/logos/ferris-hero.png" alt="" width="300" />
                </div>

                // What the hell???
                // R: The grid is composed from every brick section (each brick's top-connections)
                // to align correctly in any™ position a brick is located. The cols are just an
                // arbritrary amount of bricks that looks good in each breakpoint.
                <div
                    style="--brick: 38px 36px 34px 34px;"
                    class="grid \
                    grid-cols-[repeat(3,var(--brick))] \
                    sm:grid-cols-[repeat(4,var(--brick))] \
                    md:grid-cols-[repeat(5,var(--brick))] \
                    lg:grid-cols-[repeat(7,var(--brick))]"
                >
                    {sponsors}
                </div>
            </div>
        </section>
    }
}
