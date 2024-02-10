// Libs
use wasm_bindgen::prelude::*;

mod app;
use app::App;

// Functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Main function.
fn main() {
    yew::Renderer::<App>::new().render();
}
