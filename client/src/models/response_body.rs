// Libs
use serde::Deserialize;

// Enums
#[derive(Deserialize)]
pub enum Status {
    Success,
    Failed,
}

// Structs
#[derive(Deserialize)]
pub struct ResponseBody<T> {
    pub status: Status,
    pub data: T,
}
