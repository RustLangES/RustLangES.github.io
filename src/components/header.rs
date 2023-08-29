use leptos::*;

use crate::components::button_link::ButtonLink;

#[component]
pub fn Header() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    view! {
        <header class="border-b border-b-black/20">
            <div class="container mx-auto px-4 flex items-center justify-between flex-col lg:flex-row">
                <div class="flex justify-between w-full lg:w-auto">
                    <div class="flex items-center gap-x-4">
                        <img
                            src="https://www.rust-lang.org/static/images/rust-logo-blk.svg"
                            class="max-h-20 rounded-full"
                            alt="Rust Lang en Español"
                        />
                    </div>
                    <button
                        class="lg:hidden"
                        on:click=move |_| { set_is_open.update(|n| *n = !*n) }
                    >
                        <span class="w-6 h-1 bg-black block my-4 relative after:absolute after:block after:bg-black after:w-6 after:h-1 after:bottom-2 before:absolute before:block before:bg-black before:w-6 before:h-1 before:-bottom-2"></span>
                    </button>
                </div>
                <nav class=move || {
                    format!(
                        "w-full lg:w-auto pb-10 pt-5 lg:p-0 {}", if is_open() { "block" } else {
                        "hidden lg:block" }
                    )
                }>

                    <ul class="flex items-center gap-6 flex-col lg:flex-row lg:items-center">
                        <li>
                            <a href="https://rustlanges.github.io/rust-book-es" target="_blank">
                                "Aprende"
                            </a>
                        </li>
                        <li>
                            <a href="#">"Comunidad"</a>
                        </li>
                        <li>
                            <a href="#">"Blog"</a>
                        </li>
                        <ul class="lg:ml-4 flex items-center gap-x-6">
                            <li>
                                <ButtonLink href="https://github.com/RustLangES">
                                    "Github"
                                </ButtonLink>
                            </li>
                            <li>
                                <ButtonLink href="https://discord.gg/4ng5HgmaMg">
                                    "Discord"
                                </ButtonLink>
                            </li>
                        </ul>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
