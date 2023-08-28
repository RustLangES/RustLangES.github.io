use leptos::*;

use crate::components::icons::github_icon::GithubIcon;
use crate::components::button_link::ButtonLink;

#[component]
pub fn CommunityProjects() -> impl IntoView {
    view! {
        <section class="bg-orange-200 py-20">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl text-left mb-6">
                    <span class="font-work-sans font-light">"Proyectos de la "</span>
                    <span class="font-alfa-slab text-orange-500">"Comunidad"</span>
                </h2>
                <div class="w-full grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-x-8 gap-y-4">
                    <a
                        href="https://github.com/RustLangES/rust-book-es"
                        target="_blank"
                        class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-200 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                    >
                        <div class="flex flex-col justify-between h-full">
                            <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" width="60" class="bg-white rounded-full mb-4" />
                            <h5 class="text-xl h-full">
                                <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                    "Rust"
                                </span>
                                <span class="font-work-sans text-black">
                                    " Lang en Español"
                                </span>
                            </h5>
                            <p class="mt-2 font-work-sans text-black">
                                "Pagina de la comunidad de Rust en Español"
                            </p>
                        </div>
                        <div class="flex gap-2 items-center">
                            <ButtonLink href="https://github.com/RustLangES" size="tiny">
                                "Rust Lang en Español"
                            </ButtonLink>
                            <span class="ml-auto">
                            <GithubIcon size=30 />
                            </span>
                        </div>
                    </a>
                    <a
                        href="https://github.com/RustLangES/rust-book-es"
                        target="_blank"
                        class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-200 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                    >
                        <div class="flex flex-col justify-between h-full">
                            <img src="./RustLogo.png" width="60" class="rounded-full mb-4" />
                            <h5 class="text-xl  h-full">
                                <span class="font-work-sans text-black">
                                    "The "
                                </span>
                                <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                    "Rust"
                                </span>
                                <span class="font-work-sans text-black">
                                    " Programming Language [Spanish Ed.]"
                                </span>
                            </h5>
                            <p class="mt-2 font-work-sans text-black">
                                "Traducción del libro oficial de Rust al español"
                            </p>
                        </div>
                        <div class="flex gap-2 items-center">
                            <ButtonLink href="https://github.com/RustLangES" size="tiny">
                                "Rust Lang en Español"
                            </ButtonLink>
                            <span class="ml-auto">
                            <GithubIcon size=30 />
                            </span>
                        </div>
                    </a>
                    <a
                    href="https://github.com/datapture/hereditary"
                    target="_blank"
                    class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-200 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                >
                    <div class="flex flex-col justify-between h-full">
                        <div class="flex">
                            <div class="h-[60px] w-[60px] bg-gray-900 rounded-full mb-4 p-2 text-white text-4xl flex justify-center items-center">
                                <p>"H"</p>
                            </div>
                        </div>
                            <h5 class="text-xl  h-full">
                            <span class="font-work-sans text-black">
                                "Hereditary"
                            </span>
                        </h5>
                        <p class="mt-2 font-work-sans text-black h-full">
                            "Una libreria para emular herencia en "
                            <span class="font-alfa-slab text-orange-500 group-hover:text-white">
                                "Rust"
                            </span>
                        </p>
                    </div>
                    <div class="flex gap-2 items-center">
                        <ButtonLink href="https://github.com/superoptimo" size="tiny">
                            "Francisco Leon"
                        </ButtonLink>
                        <span class="ml-auto">
                        <GithubIcon size=30 />
                        </span>
                    </div>
                </a>
                    <a
                    href="https://github.com/whizzes/gabble"
                    target="_blank"
                    class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-200 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                >
                <div class="flex flex-col justify-between h-full">
                    <div class="flex">
                        <div class="h-[60px] w-[60px] bg-white rounded-full mb-4 p-2 text-black text-3xl flex justify-center items-center">
                            <p>"🎎"</p>
                        </div>
                    </div>
                        <h5 class="text-xl  h-full">
                        <span class="font-work-sans text-black">
                            "gabble"
                        </span>
                    </h5>
                    <p class="mt-2 font-work-sans text-black h-full">
                        "Un sistema de chats en tiempo real"
                    </p>
                </div>
                <div class="flex gap-2 items-center">
                    <ButtonLink href="https://github.com/EstebanBorai" size="tiny">
                        "Esteban Borai"
                    </ButtonLink>
                    <span class="ml-auto">
                    <GithubIcon size=30 />
                    </span>
                </div>
                </a>
                <a
                    href="https://graphul-rs.github.io"
                    target="_blank"
                    class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-200 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                >
                    <div class="flex flex-col justify-between h-full">
                        <div class="flex">
                            <img src="https://graphul-rs.github.io/img/logo.svg" width="60" height="60" class="h-[60px] w-[60px] bg-gray-900 rounded-full mb-4 p-2" />
                        </div>
                            <h5 class="text-xl  h-full">
                            <span class="font-work-sans text-black">
                                "Graphul"
                            </span>
                        </h5>
                        <p class="mt-2 font-work-sans text-black h-full">
                            "Un framework web basado en Axum"
                        </p>
                    </div>
                    <div class="flex gap-2 items-center">
                        <ButtonLink href="https://github.com/SamuelBonilla" size="tiny">
                            "Samuel Bonilla"
                        </ButtonLink>
                        <span class="ml-auto">
                        <GithubIcon size=30 />
                        </span>
                    </div>
                </a>
                <a
                    href="https://github.com/http-server-rs/http-server"
                    target="_blank"
                    class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-200 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                >
                    <div class="flex flex-col justify-between h-full">
                        <div class="flex">
                            <img src="https://github.com/http-server-rs/http-server/raw/main/assets/logo.svg" width="60" height="60" class="h-[60px] w-[60px] bg-white rounded-full mb-4 p-2" />
                        </div>
                            <h5 class="text-xl  h-full">
                            <span class="font-work-sans text-black">
                                "http-server"
                            </span>
                        </h5>
                        <p class="mt-2 font-work-sans text-black h-full">
                            "Un command line HTTP server simple y configurable"
                        </p>
                    </div>
                    <div class="flex gap-2 items-center">
                        <ButtonLink href="https://github.com/EstebanBorai" size="tiny">
                            "Esteban Borai"
                        </ButtonLink>
                        <span class="ml-auto">
                        <GithubIcon size=30 />
                        </span>
                    </div>
                </a>
                <a
                    href="https://github.com/marc2332/freya"
                    target="_blank"
                    class="group flex flex-col gap-y-6 border border-black p-6 hover:bg-orange-500 bg-orange-200 drop-shadow-[0_0_0_rgba(0,0,0)] hover:drop-shadow-[-4px_-4px_0_rgba(0,0,0)] transition justify-between"
                >
                    <div class="flex flex-col justify-between h-full">
                        <div class="flex">
                            <img src="https://github.com/marc2332/freya/raw/main/logo.svg" width="60" height="60" class="rounded-full mb-4" />
                        </div>
                            <h5 class="text-xl  h-full">
                            <span class="font-work-sans text-black">
                                "Freya"
                            </span>
                        </h5>
                        <p class="mt-2 font-work-sans text-black h-full">
                            "Una libreria para hacer GUI nativas utilizando Dioxus y Skia"
                        </p>
                    </div>
                    <div class="flex gap-2 items-center">
                        <ButtonLink href="https://github.com/marc2332" size="tiny">
                            "Marc Espín"
                        </ButtonLink>
                        <span class="ml-auto">
                        <GithubIcon size=30 />
                        </span>
                    </div>
                </a>
                </div>
            </div>
        </section>
    }
}
