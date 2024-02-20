use leptos::{component, view, IntoView};

#[component]
pub fn CardTitle(#[prop(into)] texts: &'static [&'static str]) -> impl IntoView {
    // split the vec in two when a word contains "rust"
    // and wrap it in a span with a different color
    // For example: The Rust book Spanish Edition -> ["The ", "Rust", " book Spanish Edition"]
    let mut words = texts.to_owned();
    for word in &mut words {
        if word.to_lowercase().contains("rust") {
            *word = r#"<span class="font-alfa-slab text-orange-500 group-hover:text-white">Rust</span>"#;
        }
    }

    view! { <h3 class="text-xl font-work-sans text-black" inner_html=words.join(" ")></h3> }
}
