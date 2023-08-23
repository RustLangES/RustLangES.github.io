use leptos::*;


#[component]
pub fn Header(
    cx: Scope
) -> impl IntoView {
    view!{ cx,
        <header class="w-full py-1 px-12 bg-amber-500 p-1 flex justify-between items-center drop-shadow-md">
            <div class="flex items-center">
                <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" class="max-h-20 rounded-full" />
                <h1 class="text-xl font-alfa-slab">{ "Rust Lang en Español" }</h1>
            </div>
            <nav class="flex items-center gap-5">
                <a class="text-xl font-fira-sans" href="https://github.com/RustLangES"> { "Github" } </a>
                <a class="text-xl font-fira-sans" href="https://discord.gg/4ng5HgmaMg"> { "Discord" } </a>
            </nav>
        </header>
    }
}