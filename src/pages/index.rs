use leptos::prelude::*;
use rustlanges_components::badge::{Type, Variant};
use rustlanges_components::button::{Button, Variant as ButtonVariant};

#[component]
pub fn Index() -> impl IntoView {
    let (gcount, wcount) = signal(20);
    
    view! {
        <div class="w-full h-[65dvh] rustlang-es-background text-akira flex items-center justify-center">
            <div class="w-full container xl:max-w-[110rem]  flex flex-row items-center justify-center m-auto" >
                <div class="flex flex-col justify-center gap-8" >
                    <div class="flex flex-col gap-2">
                        <p class="uppercase">Comunidad en español</p>
                        <p class="uppercase text-h1">Hacemos <span class="text-primary-500">Posible</span> Aprender Rust</p>
                    </div>
                    <div class="flex gap-4">
                        <Button variant=ButtonVariant::Secondary on_click=|_| {} >Aprender</Button>
                        <Button variant=ButtonVariant::Primary  on_click=|_| {}>"Súmate"</Button>
                    </div>
                </div>
                <div >
                    <img src="/assets/new/logos/ferris-hero.png" alt="" />
                </div>
            </div>
        </div>
    }
}
