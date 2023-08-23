

use leptos::*;


#[component]
pub fn Hero(
    cx: Scope
) -> impl IntoView {
    view!{ cx,
        <div  class=" w-full bg-no-repeat bg-center">
            <div class="my-0 mx-auto text-center flex justify-center flex-col items-center">
                <img src="./rhq3ezvso9611.png" width="400" class="mt-5" />
                <h2 class="p-6 text-4xl">"Bienvenidos a Rust Lang en Español"</h2>
                <p class="px-10 pb-10 text-left">"Una comunidad de gente mal intencionada y tonta."</p>
            </div>
        </div>
    }
}