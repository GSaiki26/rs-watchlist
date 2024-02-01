// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::{info, warn};

use super::database_model::DatabaseModel;
use crate::database::DATABASE;

// Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub title: String,
    pub description: String,
    pub watchlist: Thing,
    pub watched: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// Implementations
impl Media {
    /**
     * A method to create a new media.
     */
    fn new(title: String, description: String, watchlist: String, watched: bool) -> Self {
        Self {
            id: None,
            title,
            description,
            watched,
            watchlist: Thing {
                id: Id::from(watchlist),
                tb: String::from("watchlist"),
            },
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        }
    }
}

impl DatabaseModel<Media> for Media {
    async fn from_id(id: Id) -> surrealdb::Result<Option<Self>> {
        // Create the thing.
        let thing = Thing {
            id,
            tb: String::from("user"),
        };

        // Get the media.
        info!("Getting {}.", thing);
        match DATABASE.select::<Option<Self>>(thing).await? {
            None => {
                info!("No media found.");
                Ok(None)
            }
            Some(media) => {
                info!("media found.");
                Ok(Some(media))
            }
        }
    }

    async fn migration() -> surrealdb::Result<()> {
        // Define the media table.
        info!("Running Media migration...");
        DATABASE
            .query(
                "
                    BEGIN TRANSACTION;
                    DEFINE TABLE media SCHEMAFULL;
                    DEFINE FIELD title ON TABLE media TYPE string;
                    DEFINE FIELD description ON TABLE media TYPE string;
                    DEFINE FIELD watchlist ON TABLE media TYPE record(watchlist);
                    DEFINE FIELD watched ON TABLE media TYPE bool;
                    DEFINE FIELD created_at ON TABLE media TYPE datetime;
                    DEFINE FIELD updated_at ON TABLE media TYPE datetime;
                    COMMIT TRANSACTION;
                ",
            )
            .await?;

        Ok(())
    }

    async fn sync(&mut self) -> surrealdb::Result<()> {
        // Check if the Media already has an id. If not, generate a new one.
        if self.id.is_none() {
            info!("Generating a new ID for the media...");
            loop {
                let id = Id::ulid();
                if Self::from_id(id.clone()).await?.is_none() {
                    self.id = Some(Thing {
                        id,
                        tb: String::from("media"),
                    });
                    break;
                }
            }
            info!("Generated a new ID.");
            self.created_at = Datetime::default();
        }

        // Sync the media in the database.
        self.updated_at = Datetime::default();
        info!("Syncing the media in the database...");
        DATABASE.update::<Vec<Self>>("media").content(&self).await?;
        info!("Synced the media in the database.");

        Ok(())
    }

    async fn delete(self) -> surrealdb::Result<()> {
        // Check if the media has an id.
        if let Some(id) = self.id.clone() {
            info!("Deleting {}...", &id);
            DATABASE.delete::<Option<()>>(&id).await?;
            info!("The {} was deleted.", id);
        } else {
            warn!("The media has no id.");
        }

        Ok(())
    }
}
