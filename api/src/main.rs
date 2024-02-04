// Libs
use std::process::exit;

use tracing::{error, info};

use database::initialize_db;
mod controllers;
mod database;
mod models;
mod router;
mod security;

// Data
static PORT: i16 = 3000;

// Main function
#[tokio::main]
async fn main() {
    // Initialize the logger.
    tracing_subscriber::fmt::init();

    if let Err(e) = initialize_db().await {
        error!("Couldn\'t initialize the database. {}", e);
        exit(1);
    }

    // Open the server.
    info!("Starting server on port {}...", PORT);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT)).await;
    if let Err(e) = listener {
        error!("Couldn\'t listen on port {}. {}", PORT, e);
        exit(1);
    }

    info!("Server started successfully.");
    let listener = listener.unwrap();
    axum::serve(listener, router::get_router())
        .await
        .expect("Couldn\'t serve the port.");
}
