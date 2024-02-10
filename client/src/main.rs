// Libs
use wasm_bindgen::prelude::*;

mod app;
mod models;
use app::App;

// Functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Main function.
#[tokio::main]
async fn main() {
    yew::Renderer::<App>::new().render();
}
