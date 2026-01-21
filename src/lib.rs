pub mod app;
pub mod components;
#[rustfmt::skip]
pub mod extras;
pub mod models;
pub mod pages;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: String);

    #[wasm_bindgen(js_namespace = console)]
    fn warn(msg: String);

    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        if cfg!(debug_assertions) {
            log(format_args!($($t)*).to_string())
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($($t:tt)*) => (warn(format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! error {
    ($($t:tt)*) => (error(format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn hydrate() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicHookInfo| {
        error!("{info}")
    }));
}
