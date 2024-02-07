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

    #[serde(skip_serializing_if = "String::is_empty")]
    password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Datetime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Datetime>,
}

// Implementations
impl User {
    /**
     * A method to get an user by username.
     */
    pub async fn from_username(username: &str) -> surrealdb::Result<Option<Self>> {
        // Get the user.
        info!("Getting user {}.", username);
        match DATABASE
            .query("SELECT * FROM user WHERE username = $username")
            .bind(("username", username.to_lowercase()))
            .await?
            .take(0)
        {
            Err(e) => {
                warn!("Couldn\'t get the user. {}", e);
                Ok(None)
            }
            Ok(None) => {
                info!("No user {} found.", username);
                Ok(None)
            }
            Ok(user) => {
                info!("User {} found.", username);
                Ok(user)
            }
        }
    }

    /**
     * A method to check if the login is valid for the user.
     */
    pub fn is_login_valid(&self, passwd: String) -> bool {
        // Check if the password are equals.
        if self.password != get_sha512(passwd.as_bytes()) {
            info!("The user exists but the password is wrong.");
            return false;
        }

        true
    }

    /**
     * A method to clear the password
     */
    pub fn clear_password(&mut self) {
        self.password = String::new();
    }
}

impl ModelTrait<User> for User {
    async fn from_id(id: Id) -> surrealdb::Result<Option<Self>> {
        // Create the thing.
        let thing = Thing {
            id,
            tb: String::from("user"),
        };

        // Get the user.
        info!("Getting {}.", &thing);
        match DATABASE.select::<Option<Self>>(thing.clone()).await? {
            None => {
                info!("No {} found.", &thing);
                Ok(None)
            }
            Some(user) => {
                // Remove the password from the user. It's not safe to keep it.
                info!("{} found.", thing);
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
                    DEFINE FIELD username ON TABLE user TYPE string VALUE string::lowercase($value) ASSERT $value = /^[a-zA-Z0-9!@#$%&*_\\-+.,<>;\\/? ]{3,20}$/;
                    DEFINE FIELD password ON TABLE user TYPE string ASSERT $value = /^[a-z0-9]{128}$/;
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

        // Encrypt the password to SHA512.
        if self.password.len() != 128 {
            info!("Encrypting the password...");
            self.password = get_sha512(self.password.as_bytes());
        }

        // Sync the user in the database.
        self.updated_at = Some(Datetime::default());
        info!("Syncing {} in the database...", self.id.as_ref().unwrap());
        DATABASE.update::<Vec<Self>>("user").content(&self).await?;
        info!("Synced {} in the database.", self.id.as_ref().unwrap());

        Ok(())
    }

    async fn create(&mut self) -> surrealdb::Result<()> {
        // Generate a new thing for the user.
        info!("Creating a new user...");
        self.id = Some(Self::generate_new_ulid("user").await?);

        // Create the user in the database.
        self.created_at = Some(Datetime::default());
        self.updated_at = self.created_at.clone();
        let created_users = DATABASE.create::<Vec<Self>>("user").content(&self).await?;

        // Check if it was really created.
        if created_users.is_empty() {
            warn!("No user was created.");
            dbg!(&self);
        }

        info!("The new {} was created.", self.id.as_ref().unwrap());
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
