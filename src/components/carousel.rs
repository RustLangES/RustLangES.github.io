use leptos::{ev, html, prelude::*};
use rustlanges_components::{
    button::{Variant, button_class},
    icons::{ArrowLeft, ArrowRight},
};

// This was not planned but I think that it could be a good idea, however we must to move this component
// to the design system eventually
#[island]
pub fn Carousel(
    children: Children,
    count: usize,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let track = NodeRef::<html::Div>::new();
    let current = RwSignal::new(0);
    let overflows = RwSignal::new(false);
    let last = count.saturating_sub(1);

    // I was not secure how to this so maybe could bre wrong, the cards are all the same width, so the
    // scroll position tells us which one is in view, I translate this trick from JS to Leptos
    // in the most closed to the original
    let update = move || {
        let Some(el) = track.get_untracked() else {
            return;
        };

        let max = (el.scroll_width() - el.client_width()) as f64;
        overflows.set(max > 1.0 && last > 0);

        current.set((el.scroll_left() as f64 / max * last as f64).round() as usize);
    };

    let scroll_to = move |i: usize| {
        if let Some(el) = track.get_untracked()
            && last > 0
        {
            let max = (el.scroll_width() - el.client_width()) as f64;
            el.set_scroll_left((max * i.min(last) as f64 / last as f64) as i32);
        }
    };

    Effect::new(move |_| {
        if track.get().is_some() {
            update();
        }
    });

    let _ = window_event_listener(ev::resize, move |_| update());

    let arrow = button_class(
        Variant::Icon,
        Some("size-9 [&>svg]:size-5 disabled:opacity-30 disabled:pointer-events-none".into()),
    );

    view! {
        <div class="w-full">
            <div
                node_ref=track
                on:scroll=move |_| update()
                class=format!(
                    "flex gap-4 overflow-x-auto snap-x snap-mandatory scroll-smooth scroll-px-4 px-4 py-4 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden {}",
                    class.unwrap_or_default(),
                )
            >
                {children()}
            </div>
            <div class=move || {
                if overflows() { "flex items-center justify-center gap-4 pt-2" } else { "hidden" }
            }>
                <button
                    class=arrow.clone()
                    aria-label="Anterior"
                    disabled=move || current() == 0
                    on:click=move |_| scroll_to(current.get_untracked().saturating_sub(1))
                >
                    <ArrowLeft />
                </button>

                <div class="flex items-center gap-1.5">
                    {(0..count)
                        .map(|i| {
                            view! {
                                <button
                                    aria-label=format!("Ir al elemento {}", i + 1)
                                    on:click=move |_| scroll_to(i)
                                    class=move || {
                                        if current() == i {
                                            "h-2.5 w-6 rounded-full border border-black bg-primary-500 transition-all"
                                        } else {
                                            "size-2.5 rounded-full bg-neutral-300 dark:bg-neutral-600 transition-all"
                                        }
                                    }
                                />
                            }
                        })
                        .collect_view()}
                </div>

                <button
                    class=arrow
                    aria-label="Siguiente"
                    disabled=move || current() == last
                    on:click=move |_| scroll_to(current.get_untracked() + 1)
                >
                    <ArrowRight />
                </button>
            </div>
        </div>
    }
}
