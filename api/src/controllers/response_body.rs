// Libs
use axum::Json;
use serde::Serialize;

// Enums
#[derive(Serialize)]
enum Status {
    Success,
    Failed,
}

// Structs
#[derive(Serialize)]
pub struct ResponseBody<T: Serialize> {
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// Implementations
impl<T: Serialize> ResponseBody<T> {
    /**
     * Create a success response.
     */
    pub fn success(data: T) -> Json<Self> {
        Json(Self {
            status: Status::Success,
            data: Some(data),
            message: None,
        })
    }

    /**
     * Create a success response with no body.
     */
    pub fn success_no_data() -> Json<Self> {
        Json(Self {
            status: Status::Success,
            data: None,
            message: None,
        })
    }

    /**
     * Create an error response.
     */
    pub fn error(message: &str) -> Json<Self> {
        Json(Self {
            status: Status::Failed,
            data: None,
            message: Some(message.to_string()),
        })
    }
}
