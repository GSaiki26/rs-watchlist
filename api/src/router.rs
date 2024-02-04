use std::time::Duration;

// Libs
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::controllers::user_controler::*;

// Functions
pub fn get_router() -> Router {
    Router::new()
        .route("/user", post(create_user))
        .route("/user/:id", get(get_user_by_id))
        .route("/user/:id", patch(update_user_by_id))
        .route("/user/:id", delete(delete_user))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(10))),
        )
}
