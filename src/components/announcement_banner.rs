use leptos::prelude::*;

#[component]
pub fn AnnouncementBanner() -> impl IntoView {
    let text = "¡Últimos lugares! * ¡Últimos lugares! * ¡Últimos lugares! * ¡Últimos lugares! * ¡Últimos lugares! * ¡Últimos lugares! * ¡Últimos lugares! * ¡Últimos lugares! * ";

    view! {
        <div class="w-full overflow-hidden py-1">
            <div
                class="bg-black text-white py-3 w-full overflow-hidden"
                style="transform: skewY(-1.5deg); transform-origin: left center; margin-bottom: -2px;"
            >
                <p class="marquee-track text-lg font-bold whitespace-nowrap">{text}{text}</p>
            </div>
            <div
                class="bg-primary-500 text-white py-3 w-full overflow-hidden"
                style="transform: skewY(-1.5deg); transform-origin: left center;"
            >
                <p class="marquee-track marquee-reverse text-lg font-bold whitespace-nowrap">{text}{text}</p>
            </div>
        </div>
    }
}
