// Libs
use std::process::exit;

use tracing::error;

use database::initialize_db;
mod database;
mod models;

// Main function
#[tokio::main]
async fn main() {
    // Initialize the logger
    tracing_subscriber::fmt::init();

    if let Err(e) = initialize_db().await {
        error!("Couldn\'t initialize the database. {}", e);
        exit(1);
    }
}
