use leptos::*;

use crate::components::header::Header;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="w-full flex flex-col">
            <Header />
            <div class="flex items-center justify-center py-32">
                <div class="flex items-center gap-x-20">
                    <figure>
                        <img src="./rhq3ezvso9611.png" width="500" class="mx-auto" />
                    </figure>
                    <div class="">
                        <h1 class="flex flex-col mb-4 gap-y-2">
                            <span class="font-work-sans text-4xl font-light">"Bienvenidos a"</span>
                            <span class="font-alfa-slab text-orange-500 text-8xl">"Rust Lang"</span>
                            <span class="font-work-sans text-5xl font-semibold">"En Español"</span>
                        </h1>
                        <p class="font-work-sans font-light">
                            "Una comunidad de gente mal intencionada y tonta."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
