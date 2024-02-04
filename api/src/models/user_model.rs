// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::{info, warn};

use super::model_trait::ModelTrait;
use crate::database::DATABASE;
use crate::security::get_sha512;

// Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Datetime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
}

// Implementations
impl ModelTrait<User> for User {
    async fn from_id(id: Id) -> surrealdb::Result<Option<Self>> {
        // Create the thing.
        let thing = Thing {
            id,
            tb: String::from("user"),
        };

        // Get the user.
        info!("Getting {}.", thing);
        match DATABASE.select::<Option<Self>>(thing).await? {
            None => {
                info!("No user found.");
                Ok(None)
            }
            Some(user) => {
                // Remove the password from the user. It's not safe to keep it.
                info!("user found.");
                Ok(Some(user))
            }
        }
    }

    async fn migration() -> surrealdb::Result<()> {
        // Define the users table.
        info!("Running User migration...");
        DATABASE
            .query(
                "
                    BEGIN TRANSACTION;
                    DEFINE TABLE user SCHEMAFULL;
                    DEFINE FIELD username ON TABLE user TYPE string;
                    DEFINE FIELD password ON TABLE user TYPE string;
                    DEFINE FIELD created_at ON TABLE user TYPE datetime;
                    DEFINE FIELD updated_at ON TABLE user TYPE datetime;
                    COMMIT TRANSACTION;
                ",
            )
            .await?;

        Ok(())
    }

    async fn sync(&mut self) -> surrealdb::Result<()> {
        // Check if the user already has an id. If not, generate a new one.
        if self.id.is_none() {
            return self.create().await;
        }

        // Sync the user in the database.
        self.updated_at = Some(Datetime::default());
        info!("Syncing the user in the database...");
        DATABASE.update::<Vec<Self>>("user").content(&self).await?;
        info!("Synced the user in the database.");

        Ok(())
    }

    async fn create(&mut self) -> surrealdb::Result<()> {
        // Generate a new thing for the user.
        info!("Creating a new user...");
        self.id = Some(Self::generate_new_ulid("user").await?);

        // Encrypt the password to SHA512.
        info!("Encrypting the password...");
        self.password = get_sha512(self.password.as_bytes());

        // Create the user in the database.
        self.created_at = Some(Datetime::default());
        self.updated_at = self.created_at.clone();
        let created_users = DATABASE.create::<Vec<Self>>("user").content(&self).await?;

        // Check if it was really created.
        if created_users.is_empty() {
            warn!("No user was created.");
            dbg!(self);
        }

        info!("New user created.");
        Ok(())
    }

    async fn delete(self) -> surrealdb::Result<()> {
        // Check if the user has an id.
        if let Some(id) = self.id.clone() {
            info!("Deleting {}...", &id);
            DATABASE.delete::<Option<User>>(&id).await?;
            info!("The {} was deleted.", id);
        } else {
            warn!("The user has no id.");
        }

        Ok(())
    }
}
