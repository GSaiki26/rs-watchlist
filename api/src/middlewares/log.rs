use std::net::SocketAddr;

// Libs
use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::Response,
    RequestExt,
};
use tracing::info;

// Functions
pub async fn log_stream_middleware(mut req: Request, next: Next) -> Response {
    let ip = req
        .extract_parts::<ConnectInfo<SocketAddr>>()
        .await
        .expect("Couldn\'t get the connection information.");
    info!(
        "Request to from [{}] to {} {}",
        ip.to_string(),
        req.method(),
        req.uri()
    );
    let res = next.run(req).await;
    info!(
        "Returning response #{} to the client [{}].",
        res.status(),
        ip.to_string()
    );
    res
}
