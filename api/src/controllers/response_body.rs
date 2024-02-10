// Libs
use axum::Json;
use serde::Serialize;
use serde_json::Value;

// Enums
#[derive(Serialize)]
enum Status {
    Success,
    Failed,
}

// Structs
#[derive(Serialize)]
pub struct ResponseBody {
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// Implementations
impl ResponseBody {
    /**
     * A method to create a success response body.
     */
    pub fn success<T: Serialize>(data: T) -> Json<Self> {
        Json(Self {
            status: Status::Success,
            data: Some(serde_json::to_value(data).unwrap()),
            message: None,
        })
    }

    /**
     * A method to create a success response without the data field.
     */
    pub fn success_no_data() -> Json<Self> {
        Json(Self {
            status: Status::Success,
            data: None,
            message: None,
        })
    }

    /**
     * A method to create a error response body.

    */
    pub fn error(message: &str) -> Json<Self> {
        Json(Self {
            status: Status::Failed,
            data: None,
            message: Some(message.to_string()),
        })
    }
}
