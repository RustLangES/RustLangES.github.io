use std::collections::HashMap;
use leptos::*;

#[component]
pub fn ButtonLink(
    href: String,
    #[prop(default = "primary".to_string())] color: String,
    #[prop(default = "normal".to_string())] size: String,
    children: Children,
) -> impl IntoView {
    let colors = HashMap::from([
        ("primary".to_string(), "bg-orange-300 hover:bg-black hover:text-white"),
        ("white".to_string(), "bg-orange-100")
    ]);
    let sizes = HashMap::from([
        ("normal".to_string(), "h-9"),
        ("big".to_string(), "h-12")
    ]);
    let current_color = colors.get(&color).unwrap().to_string();
    let current_size = sizes.get(&size).unwrap().to_string();

    view! {
        <a
            href=href.to_string()
            target="_blank"
            class=format!("tracking-widest font-work-sans border border-black flex items-center px-4 drop-shadow-[4px_4px_0_rgba(0,0,0)] hover:drop-shadow-[0_0_0_rgba(0,0,0)] transition w-fit gap-x-4 {} {}", current_color, current_size)
        >
            {children()}
        </a>
    }
}
