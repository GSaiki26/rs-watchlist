// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::{info, warn};

use super::database_model::DatabaseModel;
use crate::database::DATABASE;

// Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// Implementations
impl User {
    /**
     * A method to create a new user.
     */
    fn new(username: String, password: String) -> Self {
        Self {
            id: None,
            username,
            password,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        }
    }
}
impl DatabaseModel<User> for User {
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
                    DEFINE FIELD username ON TABLE users TYPE string;
                    DEFINE FIELD password ON TABLE users TYPE string;
                    DEFINE FIELD created_at ON TABLE users TYPE datetime;
                    DEFINE FIELD updated_at ON TABLE users TYPE datetime;
                    COMMIT TRANSACTION;
                ",
            )
            .await?;

        Ok(())
    }

    async fn sync(&mut self) -> surrealdb::Result<()> {
        // Check if the user already has an id. If not, generate a new one.
        if self.id.is_none() {
            info!("Generating a new ID for the user...");
            loop {
                let id = Id::ulid();
                if Self::from_id(id.clone()).await?.is_none() {
                    self.id = Some(Thing {
                        id,
                        tb: String::from("user"),
                    });
                    break;
                }
            }
            info!("Generated a new ID.");
            self.created_at = Datetime::default();
        }

        // Sync the user in the database.
        self.updated_at = Datetime::default();
        info!("Syncing the user in the database...");
        DATABASE.update::<Vec<Self>>("user").content(&self).await?;
        info!("Synced the user in the database.");

        Ok(())
    }

    async fn delete(self) -> surrealdb::Result<()> {
        // Check if the user has an id.
        if let Some(id) = self.id.clone() {
            info!("Deleting {}...", &id);
            DATABASE.delete::<Option<()>>(&id).await?;
            info!("The {} was deleted.", id);
        } else {
            warn!("The user has no id.");
        }

        Ok(())
    }
}
