// Libs
use std::time::Duration;

use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use http::{header::AUTHORIZATION, Method};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};

use crate::{
    controllers::media_controler::*, controllers::user_controler::*,
    controllers::watchlist_controler::*, middlewares::acceptable_middleware::acceptable_headers,
    middlewares::log_middleware::log_stream,
};

// Functions
pub fn get_router() -> Router {
    // Define the middlewares.
    let timeout = TimeoutLayer::new(Duration::from_secs(10));
    let cors = CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
        ])
        // .allow_origin(vec!["http://localhost:1420".parse().unwrap()])
        .allow_origin(Any)
        // .allow_credentials(true)
        .allow_headers([AUTHORIZATION]);

    // Create the router.
    Router::new()
        .route("/media", post(post_media))
        .route("/media/:media_id", patch(patch_media))
        .route("/media/:media_id", delete(delete_media))
        .route("/media/:media_id", get(get_media))
        .route("/user", post(post_user))
        .route("/user", patch(patch_user))
        .route("/user", delete(delete_user))
        .route("/user/:user_id", get(get_user))
        .route("/user/login", post(post_user_login))
        .route("/watchlist", post(post_watchlist))
        .route("/watchlist", get(get_watchlists))
        .route("/watchlist/:watchlist_id", get(get_watchlist))
        .route("/watchlist/:watchlist_id", patch(patch_watchlist))
        .route("/watchlist/:watchlist_id", delete(delete_watchlist))
        .route("/watchlist/:watchlist_id/media", get(get_watchlist_medias))
        .layer(middleware::from_fn(log_stream))
        .layer(middleware::from_fn(acceptable_headers))
        .layer(ServiceBuilder::new().layer(timeout).layer(cors))
}
