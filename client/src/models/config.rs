// Libs
use std::path::Path;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::invoke;

// Data
static CONFIG_FILE: &str = "./config.json";

// Structs
pub struct Config;

#[derive(Default, Deserialize, Serialize)]
struct ConfigFile {
    server_addr: String,
}

// Implementations
impl Config {
    /**
     * A method to get the server address from the config file.
     */
    pub async fn get_server_addr() -> String {
        let default_addr = String::from("http://localhost:3000");
        if !Path::new(CONFIG_FILE).exists() {
            Self::create_file().await;
            return default_addr;
        }

        // Open the file and get the server address.
        let file = match std::fs::File::open(CONFIG_FILE) {
            Ok(file) => file,
            Err(e) => {
                console::log_1(&format!("Couldn\'t open config file: {}", e).into());
                return default_addr;
            }
        };

        // Convert the file to a ConfigFile struct.
        match serde_json::from_reader(file) {
            Ok(config) => config,
            Err(e) => {
                console::log_1(&format!("Couldn\'t read config file: {}", e).into());
                default_addr
            }
        }
    }

    /**
     * A method to create the config file.
     */
    pub async fn create_file() {
        // Call the tauri command create_file
        let arg = JsValue::from(CONFIG_FILE);
        invoke("create_config_file", arg).await;
    }
}
