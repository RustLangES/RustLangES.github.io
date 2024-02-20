use leptos::{component, view, Children, IntoView};
use std::collections::HashMap;

#[component]
pub fn ButtonLink(
    #[prop(into)] href: String,
    #[prop(default = "primary")] color: &'static str,
    #[prop(default = "normal")] size: &'static str,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let colors = HashMap::from([
        ("primary", "bg-orange-200 hover:bg-black hover:text-white"),
        ("white", "bg-orange-100"),
    ]);
    let sizes = HashMap::from([("tiny", "min-h-7"), ("normal", "h-9"), ("big", "h-12")]);
    let current_color = (*colors.get(&color).unwrap()).to_string();
    let current_size = (*sizes.get(&size).unwrap()).to_string();

    view! {
        <a
            href=href
            target="_blank"
            class=format!(
                "tracking-wider font-work-sans border border-black flex items-center px-4 drop-shadow-[4px_4px_0_rgba(0,0,0)] hover:drop-shadow-[0_0_0_rgba(0,0,0)] transition w-fit gap-x-4 sm:whitespace-nowrap max-w-[10rem] sm:max-w-none {} {} {}",
                current_color,
                current_size,
                class,
            )
        >

            {children()}
        </a>
    }
}
