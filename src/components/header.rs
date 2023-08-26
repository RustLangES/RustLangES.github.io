use leptos::*;

use crate::components::button_link::ButtonLink;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="border-b border-b-black/20">
            <div class="container mx-auto px-4 flex items-center justify-between">
                <div class="flex items-center gap-x-4">
                    <img
                        src="https://www.rust-lang.org/static/images/rust-logo-blk.svg"
                        class="max-h-20 rounded-full"
                        alt="Rust Lang en Español"
                    />
                </div>
                <nav>
                    <ul class="flex items-center gap-5">
                        <li>
                            <a href="https://rustlanges.github.io/rust-book-es">"Aprende"</a>
                        </li>
                        <li>
                            <a href="#">"Comunidad"</a>
                        </li>
                        <li>
                            <a href="#">"Blog"</a>
                        </li>
                        <li class="ml-4">
                            <ButtonLink
                                href="https://github.com/RustLangES".to_string()
                            >
                                "Github"
                            </ButtonLink>
                        </li>
                        <li>
                            <ButtonLink
                                href="https://discord.gg/4ng5HgmaMg".to_string()
                            >
                                "Discord"
                            </ButtonLink>
                        </li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
