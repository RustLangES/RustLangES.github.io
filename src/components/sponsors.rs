use std::collections::HashMap;

use leptos::{component, view, Children, IntoView, View};

use crate::components::{CloudflareIcon, Separator};

#[component]
pub fn Sponsors() -> impl IntoView {
    view! {
        <div class="bg-orange-50 dark:bg-transparent">
            <div class="max-w-full overflow-clip">
                <Separator/>
            </div>
            <section class="bg-orange-400/30 dark:bg-gray-800/50">
                <div class="container mx-auto py-20 px-8">
                    <h2 class="text-3xl text-left mb-2">"Respaldados por"</h2>
                    <h3 class="text-lg text-left mb-6">
                        "Estas son algunas organizaciones que apoyan nuestro trabajo."
                    </h3>
                    <div class="w-full grid sm:grid-cols-2 md:grid-cols-3 gap-x-8 gap-y-8">
                        <SponsorCard
                            name="Nerdearla"
                            image="https://nerdear.la/static/img/logos/n-detailed.svg"
                            link="https://nerdear.la/es/"
                            description="El evento de tecnología más grande en LatinoAmerica ha estado presentando charlas de la comunidad por 4 años seguidos."
                        />
                        <SponsorCard
                            name="Shuttle"
                            link="https://shuttle.rs"
                            image="assets/sponsors/shuttle.webp"
                            description="La empresa #1 en PAAS para Rust ha estado hosteando voluntariamente algunos servicios de la comunidad como el Cangrebot y la API."
                        />
                        <SponsorCard
                            name="Universidad Nur"
                            link="https://www.nur.edu"
                            image="assets/sponsors/universidad_nur.png"
                            description="Esta universidad ha presentado un especial interés por Rust y la comunidad, nos han dado lugar a la divulgación con charlas y talleres."
                        />
                    </div>
                    <div class="w-full flex flex-col md:flex-row items-center justify-center gap-x-8 gap-y-8">
                        <SponsorCard
                            name="Cloudflare"
                            link="https://www.cloudflare.com/es-es/"
                            component=CloudflareIcon().into_view()
                            description="Cloudflare patrocina nuestra infraestructura y servicios"
                        />
                        <SponsorCard
                            name="CrabNebula"
                            link="https://crabnebula.dev"
                            image="assets/sponsors/crabnebula.png"
                            description="Agilice el ciclo de vida de desarrollo de sus aplicaciones, facilitando más que nunca su empaquetado y envío."
                        />
                        <SponsorCard
                            name="Tauri"
                            link="https://tauri.app"
                            image="assets/sponsors/tauri.png"
                            description="Cree una aplicación optimizada, segura e independiente del frontend para su implantación multiplataforma."
                        />
                    </div>
                    <div class="w-full flex flex-col md:flex-row items-center justify-center gap-x-8 gap-y-8">
                        <SponsorCard
                            name="Cloudflare"
                            link="https://heavyduty.builders"
                            image="assets/sponsors/heavydutybuilder.png"
                            description="Queremos unir fuerzas con más constructores como nosotros en el ecosistema Blockchain, llámalo DAO si quieres, pero queremos decir mucho más."
                        />
                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
pub fn SponsorCard(
    #[prop(into)] name: &'static str,
    #[prop(into)] link: &'static str,
    #[prop(into)] description: &'static str,
    #[prop(into, default=None)] image: Option<&'static str>,
    #[prop(into, default=None)] component: Option<View>,
    #[prop(into, default = "black")] image_bg_color: &'static str,
) -> impl IntoView {
    let colors = HashMap::from([
        ("white", "bg-white dark:bg-white text-black dark:text-black"),
        ("black", "bg-black text-white dark:text-black"),
    ]);
    let current_color = (*colors.get(&image_bg_color).unwrap()).to_string();

    view! {
        <div class="group flex flex-col gap-y-6 p-6 justify-between items-center max-w-screen-sm w-full">
            <div class="mx-auto text-center">
                {image
                    .map(|image| {
                        view! {
                            <div class=format!(
                                "mx-auto rounded-full h-[120px] w-[120px] {} mb-4",
                                current_color,
                            )>
                                <Anchor link=link>
                                    <img
                                        alt=name
                                        width="120"
                                        height="120"
                                        class="mb-4 rounded-full max-w-full max-h-full"
                                        loading="lazy"
                                        src=image
                                    />
                                </Anchor>
                            </div>
                        }
                    })}
                {component
                    .map(|component| {
                        view! {
                            <div class=format!(
                                "mx-auto text-center rounded-full h-[120px] w-[120px] {} mb-4",
                                current_color,
                            )>
                                <Anchor link=link>{component}</Anchor>
                            </div>
                        }
                    })}
                <h3 class="text-xl font-bold font-work-sans text-black dark:text-white mb-2">
                    <Anchor link=link>{name}</Anchor>
                </h3> <p class="text-balance">{description}</p>
            </div>
        </div>
    }
}

#[component]
pub fn Anchor(#[prop(into)] link: &'static str, children: Children) -> impl IntoView {
    view! {
        <a href=link target="_blank">
            {children()}
        </a>
    }
}
