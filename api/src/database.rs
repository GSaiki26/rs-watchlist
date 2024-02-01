// Libs
use std::env::var;

use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tracing::info;

use crate::models::{
    database_model::DatabaseModel, media::Media, user::User, watchlist::Watchlist,
};

// Data
pub static DATABASE: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

// Functions
/**
 * A method to initialize the database.
*/
pub async fn initialize_db() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the database.
    info!("Initializing the database.");
    signin(
        &var("DATABASE_URI")?,
        &var("DATABASE_USER")?,
        &var("DATABASE_PASS")?,
    )
    .await?;

    // Run the migrations
    migrations().await?;

    info!("Successfully initialized the database.");
    Ok(())
}

/**
 * A method to connect and signin into the database.
 */
async fn signin(uri: &str, username: &str, password: &str) -> surrealdb::Result<()> {
    // Connect
    info!("Connecting to the database...");
    DATABASE.connect::<Ws>(uri).await?;

    // Signin as root.
    info!("Signing in in the database...");
    DATABASE.signin(Root { username, password }).await?;

    info!("Successfully signed in in the database.");
    Ok(())
}

/**
 * A method to run the migrations.
 */
async fn migrations() -> surrealdb::Result<()> {
    User::migration().await?;
    Media::migration().await?;
    Watchlist::migration().await
}