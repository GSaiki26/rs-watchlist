// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// let new_msg = invoke("greet", args).await.as_string().unwrap();
// Libs
use wasm_bindgen::prelude::*;

mod components;
mod models;
mod pages;
mod router;

// Functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Main function.
fn main() {
    yew::Renderer::<router::App>::new().render();
}
