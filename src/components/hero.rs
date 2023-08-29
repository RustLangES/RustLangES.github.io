use leptos::*;

use crate::components::header::Header;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="w-full flex flex-col">
            <Header/>
            <div class="flex items-center justify-center py-14 lg:py-32 px-4">
                <div class="grid items-center gap-x-20 gap-y-10 lg:grid-cols-2">
                    <figure class="w-80 mx-auto lg:w-full">
                        <img src="./rhq3ezvso9611-min.png" width="500" class="mx-auto"/>
                    </figure>
                    <div class="">
                        <h1 class="flex flex-col mb-4 gap-y-2">
                            <span class="font-work-sans text-4xl font-light text-center lg:text-left">
                                "Bienvenidos a"
                            </span>
                            <span class="font-alfa-slab text-orange-500 text-6xl sm:text-7xl lg:text-8xl text-center lg:text-left">
                                "Rust Lang"
                            </span>
                            <span class="font-work-sans text-5xl font-semibold text-center lg:text-left">
                                "En Español"
                            </span>
                        </h1>
                        <p class="font-work-sans font-light text-center lg:text-left">
                            "Una comunidad de gente mal intencionada y tonta."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
