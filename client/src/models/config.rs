// Libs
use serde::{Deserialize, Serialize};

// Structs
pub struct Config<'a> {
    pub filepath: &'a str,
}

#[derive(Deserialize, Serialize)]
struct ConfigFile {
    server_addr: String,
}

// Implementations
impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        Config {
            filepath: "./config.json",
        }
    }

    /**
     * A method to get the server address from the config file.
     */
    pub fn get_server_addr(&self) -> String {
        let file = std::fs::File::open(self.filepath).unwrap();
        match serde_json::from_reader(file) {
            Ok(config) => config,
            Err(_) => String::from("http://localhost:3000"),
        }
    }
}
