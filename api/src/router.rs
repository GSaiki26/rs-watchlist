use std::time::Duration;

// Libs
use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;

use crate::controllers::user_controler::*;
// use crate::controllers::watchlist_controler::*;
use crate::middlewares::log::log_stream_middleware;

// Functions
pub fn get_router() -> Router {
    Router::new()
        .route("/user", post(post_user))
        .route("/user", patch(patch_user))
        .route("/user", delete(delete_user))
        .route("/user/:user_id", get(get_user))
        .route("/user/login", delete(post_user_login))
        // .route("/watchlist", post(create_watchlist))
        // .route("/watchlist/:id", get(get_watchlist))
        // .route("/watchlist", patch(update_watchlist))
        // .route("/watchlist", delete(delete_watchlist))
        .layer(middleware::from_fn(log_stream_middleware))
        .layer(ServiceBuilder::new().layer(TimeoutLayer::new(Duration::from_secs(10))))
}
