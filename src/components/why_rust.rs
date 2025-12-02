use leptos::prelude::*;
use rustlanges_components::{
    avatar::Avatar,
    button::{Button, Variant as ButtonVariant},
    card::Card,
};

pub fn WhyRust() -> impl IntoView {
    view! {
        <section class="bg-light dark:bg-dark p-4 lg:p-20 w-full">
            <div class="container xl:max-w-7xl lg:px-36 mx-auto flex flex-row gap-16 flex-wrap md:flex-nowrap justify-center items-center">
                <div class="w-full md:w-1/2 flex flex-col gap-8 p-8">
                    <h2 class="text-5xl font-heading-1 font-bold text-center md:text-left">
                        Nosotros
                    </h2>
                    <p class="text-heading-2 font-heading-4 font-semibold">
                        Somos una <span class="text-primary-500">comunidad de Rust hispana</span>
                        que busca la promoción de este lenguaje.
                    </p>
                </div>

                <div class="w-full md:w-1/2 flex flex-col gap-8">
                    <Card class="p-0 overflow-clip min-w-xs md:min-w-md">
                        <h3 class="text-heading-3 font-heading-3 font-bold text-akira bg-neutral-950 text-center p-4 dark:text-[#FF6711]">
                            "¿Por qué Rust?"
                        </h3>
                        <p class="text-heading-3 font-body p-8">
                            Rust es un lenguaje de programación de sistemas que se enfoca en la seguridad, la velocidad y la concurrencia. Su sintaxis es similar a la de C++, pero introduce conceptos nuevos que lo hacen más seguro y más fácil de usar.
                        </p>
                    </Card>
                </div>
            </div>
        </section>
    }
}
