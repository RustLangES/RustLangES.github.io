use leptos::*;

#[component]
pub fn CardTitle(
    #[prop(into)] texts: Vec<&'static str>
) -> impl IntoView {
    view! {
        <h5 class="text-xl">
            {texts.into_iter()
                .map(|word|
                    if word.to_lowercase().contains("rust") {
                        view! {
                            <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                {word}
                            </span>
                        }
                    } else {
                        view! {
                            <span class="font-work-sans text-black">
                                {word}
                            </span>
                        }
                    }
                )
                .collect::<Vec<_>>()}
        </h5>
    }
}
