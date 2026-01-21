use leptos::{island, prelude::*, view, IntoView};

use crate::components::ButtonLink;

use crate::components::icons::LogoRustPageIcon;

#[island]
pub fn Header() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <header class="border-b border-b-black/20">
            <div class="container mx-auto px-4 flex items-center justify-between flex-col lg:flex-row">
                <div class="flex justify-between w-full lg:w-auto">
                    <a href="/" class="flex items-center gap-x-4">
                        <LogoRustPageIcon size=80 />
                    </a>
                    <button
                        class="lg:hidden"
                        on:click=move |_| { set_is_open.update(|n| *n = !*n) }
                        aria-label="Menu de opciones"
                    >
                        <span class="w-6 h-1 bg-black dark:bg-white block my-4 relative after:absolute after:block after:bg-black dark:after:bg-white after:w-6 after:h-1 after:bottom-2 before:absolute before:block before:bg-black dark:before:bg-white before:w-6 before:h-1 before:-bottom-2"></span>
                    </button>
                </div>
                <nav class=move || {
                    format!(
                        "w-full lg:w-auto pb-10 pt-5 lg:p-0 {}",
                        if is_open() { "block" } else { "hidden lg:block" },
                    )
                }>
                    <ul class="flex items-center gap-6 flex-col lg:flex-row lg:items-center">
                        <li class="nav-item">
                            <a href="https://book.rustlang-es.org" target="_blank">
                                "El Libro"
                            </a>
                        </li>
                        <li class="nav-item">
                            <a href="/aprende">"Aprende"</a>
                        </li>
                        <li class="nav-item">
                            <a href="/comunidades">"Comunidades"</a>
                        </li>

                        <li class="nav-item">
                            <a href="/colaboradores">"Colaboradores"</a>
                        </li>
                        <li class="nav-item">
                            <a href="https://blog.rustlang-es.org/" target="_self">
                                "Blog"
                            </a>
                        </li>
                        <li>
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
                        </li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
