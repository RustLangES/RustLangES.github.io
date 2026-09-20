use std::cell::RefCell;

use leptos::prelude::*;
use rand::seq::SliceRandom;
use wasm_bindgen::UnwrapThrowExt;

use crate::components::icons::SponsorBlockIcon;

const GHOST_DEBUG: bool = false;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum SponsorVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    Black,
    White,
    Custom(&'static str),
}

impl SponsorVariant {
    pub fn random_colored() -> Self {
        thread_local! {
            static COLOR_DISTRIBUTION: RefCell<Vec<SponsorVariant>> = RefCell::new(Vec::new());
        }

        COLOR_DISTRIBUTION.with_borrow_mut(|c| {
            if let Some(color) = c.pop() {
                color
            } else {
                *c = vec![
                    Self::Primary,
                    Self::Secondary,
                    Self::Tertiary,
                    Self::Quaternary,
                    Self::Primary,
                    Self::Secondary,
                    Self::Tertiary,
                    Self::Quaternary,
                ];

                c.shuffle(&mut rand::rng());

                c.pop().expect_throw("Just filled up")
            }
        })
    }
}

#[component]
pub fn SponsorBlock(
    #[prop(into, optional)] class: &'static str,
    #[prop(optional)] variant: SponsorVariant,
    #[prop(optional)] index: usize,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let (variant_custom, variant_class) = match variant {
        SponsorVariant::Primary => (
            None,
            if GHOST_DEBUG {
                "text-primary-500/5"
            } else {
                "text-primary-500"
            },
        ),
        SponsorVariant::Secondary => (
            None,
            if GHOST_DEBUG {
                "text-secondary-500/5"
            } else {
                "text-secondary-500"
            },
        ),
        SponsorVariant::Tertiary => (
            None,
            if GHOST_DEBUG {
                "text-primary-200/5"
            } else {
                "text-primary-200"
            },
        ),
        SponsorVariant::Quaternary => (
            None,
            if GHOST_DEBUG {
                "text-secondary-300/5"
            } else {
                "text-secondary-300"
            },
        ),
        SponsorVariant::Black => (
            None,
            if GHOST_DEBUG {
                "text-black/5"
            } else {
                "text-black"
            },
        ),
        SponsorVariant::White => (
            None,
            if GHOST_DEBUG {
                "text-white/5"
            } else {
                "text-white"
            },
        ),
        SponsorVariant::Custom(s) => (Some(s), "text-(--custom-color)"),
    };

    view! {
        <a
            href="#"
            style=format!(
                "--zindex: {index}; --custom-color: {};",
                variant_custom.unwrap_or("none"),
            )
            class=format!(
                "relative col-span-4 w-[143px] h-[64px] z-(--zindex) hover:z-100 hover:cursor-pointer {class}",
            )
        >
            <div class="absolute p-2 w-full h-full flex justify-center items-center">
                {children.map(|v| v())}
            </div>
            <SponsorBlockIcon class=format!("-mt-[10px] max-w-max {variant_class}") />
        </a>
    }.into_any()
}
