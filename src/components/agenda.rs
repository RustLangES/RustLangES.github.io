use leptos::prelude::*;
use rustlanges_components::{
    calendar::{Calendar, NaiveDate, Selection},
    icons::Calendar as CalendarIcon,
};

const EVENTS: [(&str, &str); 2] = [
    ("Masterclass RustLangES", "bg-primary-500"),
    ("Conferencia 26", "bg-primary-300"),
];

#[component]
pub fn Agenda() -> impl IntoView {
    view! {
        <section class="w-full py-20 dark:bg-[#333333] flex justify-center">
            <div class="container max-w-6xl px-6 flex flex-col md:flex-row items-center justify-between gap-12">
                <div class="flex flex-col gap-6 max-w-sm text-center md:text-left">
                    <h2 class="text-heading-2 font-heading-2 text-3xl font-bold">"Agenda 2026"</h2>
                    <p class="font-heading-4 text-lg leading-loose">
                        "Revisa nuestro calendario y anticípate a reservar tu lugar"
                    </p>
                </div>
                <div class="flex flex-col lg:flex-row items-center lg:items-start gap-6">
                    <AgendaCalendar />
                    <ul class="flex flex-col gap-3 w-72 max-w-full">
                        {EVENTS
                            .map(|(name, bg)| {
                                view! {
                                    <li class=format!(
                                        "flex items-center gap-3 rounded-full border-2 border-black px-4 py-2 font-medium text-black {bg}",
                                    )>
                                        <CalendarIcon size=20 />
                                        {name}
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>
            </div>
        </section>
    }
}

#[island]
fn AgendaCalendar() -> impl IntoView {
    let selected = RwSignal::new(NaiveDate::from_ymd_opt(2026, 10, 21));

    view! {
        <Calendar
            selection=Selection::Single(selected)
            default_month=NaiveDate::from_ymd_opt(2026, 8, 30).unwrap()
        />
    }
}
