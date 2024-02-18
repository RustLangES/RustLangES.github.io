use leptos::{IntoView, IntoAttribute, component, tracing, view};

use crate::components::ButtonLink;

#[component]
pub fn Roadmap(
) -> impl IntoView {
    let assets_folder = if cfg!(debug_assertions) {
        "./assets"
    }else {
        "."
    };

    view! {
        <section class="bg-orange-200 py-20 flex flex-col md:flex-row items-center justify-center container mx-auto gap-10">
            <div class="container mx-auto flex flex-wrap text-balance gap-5  w-1/2">
                <h1 class="font-alfa-slab text-3xl sm:text-4xl lg:text-5xl text-center lg:text-left">
                    "Roadmap de aprendizaje del lenguaje"
                </h1>
                <p>
                    "Navega hacia la maestría en Rust con este roadmap cuidadosamente diseñado. Desde los fundamentos hasta el desarrollo de sistemas y contribuciones a la comunidad, este plan de aprendizaje te guiará con éxito. ¡Descubre el camino hacia la excelencia en el lenguaje de programación Rust! "
                </p>
                <ButtonLink href="https://roadmap.sh/r?id=65368278035e8d1be72b3dec" size="big">
                    "Ir al Roadmap"
                </ButtonLink>
            </div>
            <div class=" w-1/2 lg:min-h-[500px]">
                <div class="image-container w-full">
                    <img
                        src=format!("{}/roadmap.avif", assets_folder)
                        alt="Roadmap"
                        width="748"
                        height="515"
                        loading="eager"
                        class="skew-y-2 absolute -left-8 -top-5 z-30"
                    />
                    <img
                        src=format!("{}/roadmap2.avif", assets_folder)
                        alt="Roadmap"
                        width="748"
                        height="515"
                        class="skew-y-2 absolute z-0 left-0 top-0"
                    />
                    <img
                        src=format!("{}/roadmap3.avif", assets_folder)
                        alt="Roadmap"
                        width="748"
                        height="515"
                        class="skew-y-2 absolute z-0 left-10 top-10"
                    />
                </div>
            </div>
        </section>
    }
}