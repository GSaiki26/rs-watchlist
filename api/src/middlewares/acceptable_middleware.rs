// Libs
use axum::{extract::Request, middleware::Next, response::Response};

// Functions
pub async fn acceptable_headers(req: Request, next: Next) -> Response {
    // Get the request headers
    let headers = req.headers();
    // Check if the request has the header "Accept"
    if headers.contains_key("Accept") {
        // Get the value of the header "Accept"
        let accept = headers.get("Accept").unwrap();
        // Check if the value of the header "Accept" is "application/json"
        if accept != "application/json" && accept != "*/*" {
            let body = axum::body::Body::empty();
            let res = Response::builder().status(406).body(body).unwrap();
            return res;
        }
    }

    next.run(req).await
}
