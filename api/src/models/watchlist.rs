// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::{info, warn};

use super::database_model::DatabaseModel;
use crate::database::DATABASE;

// Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct Watchlist {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub owner: Thing,
    pub description: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// Implementations
impl Watchlist {
    /**
     * A method to create a new watchlist.
     */
    fn new(owner: String, description: String) -> Self {
        Self {
            id: None,
            owner: Thing {
                id: Id::from(owner),
                tb: String::from("user"),
            },
            description,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        }
    }
}

impl DatabaseModel<Watchlist> for Watchlist {
    async fn from_id(id: Id) -> surrealdb::Result<Option<Self>> {
        // Create the thing.
        let thing = Thing {
            id,
            tb: String::from("watchlist"),
        };

        // Get the watchlist.
        info!("Getting {}.", thing);
        match DATABASE.select::<Option<Self>>(thing).await? {
            None => {
                info!("No watchlist found.");
                Ok(None)
            }
            Some(watchlist) => {
                info!("watchlist found.");
                Ok(Some(watchlist))
            }
        }
    }

    async fn migration() -> surrealdb::Result<()> {
        // Define the watchlist table.
        info!("Running watchlist migration...");
        DATABASE
            .query(
                "
                    BEGIN TRANSACTION;
                    DEFINE TABLE watchlist SCHEMAFULL;
                    DEFINE FIELD owner ON TABLE watchlist TYPE record(user);
                    DEFINE FIELD description ON TABLE watchlist TYPE string;
                    DEFINE FIELD created_at ON TABLE watchlist TYPE datetime;
                    DEFINE FIELD updated_at ON TABLE watchlist TYPE datetime;
                    COMMIT TRANSACTION;
                ",
            )
            .await?;

        Ok(())
    }

    async fn sync(&mut self) -> surrealdb::Result<()> {
        // Check if the watchlist already has an id. If not, generate a new one.
        if self.id.is_none() {
            info!("Generating a new ID for the watchlist...");
            loop {
                let id = Id::ulid();
                if Self::from_id(id.clone()).await?.is_none() {
                    self.id = Some(Thing {
                        id,
                        tb: String::from("watchlist"),
                    });
                    break;
                }
            }
            info!("Generated a new ID.");
            self.created_at = Datetime::default();
        }

        // Sync the watchlist in the database.
        self.updated_at = Datetime::default();
        info!("Syncing the watchlist in the database...");
        DATABASE
            .update::<Vec<Self>>("watchlist")
            .content(&self)
            .await?;
        info!("Synced the watchlist in the database.");

        Ok(())
    }

    async fn delete(self) -> surrealdb::Result<()> {
        // Check if the watchlist has an id.
        if let Some(id) = self.id.clone() {
            info!("Deleting {}...", &id);
            DATABASE.delete::<Option<()>>(&id).await?;
            info!("The {} was deleted.", id);
        } else {
            warn!("The watchlist has no id.");
        }

        Ok(())
    }
}
