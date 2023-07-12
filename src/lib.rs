use cfg_if::cfg_if;

pub mod fallback;
pub mod views;
pub mod models;
pub mod server_fn;


// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::views::app::App;
        use leptos::view;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(|cx| {
                view! { cx,  <App/> }
            });
        }
    }
}
