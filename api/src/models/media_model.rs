// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::{info, warn};

use super::model_trait::ModelTrait;
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
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

// Implementations
impl ModelTrait<Media> for Media {
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
            return self.create().await;
        }

        // Sync the media in the database.
        self.updated_at = Some(Datetime::default());
        info!("Syncing the media in the database...");
        DATABASE.update::<Vec<Self>>("media").content(&self).await?;
        info!("Synced the media in the database.");

        Ok(())
    }

    async fn create(&mut self) -> surrealdb::Result<()> {
        // Generate a new thing for the media.
        info!("Creating a new media...");
        self.id = Some(Self::generate_new_ulid("media").await?);

        // Create the media in the database.
        self.created_at = Some(Datetime::default());
        self.updated_at = self.created_at.clone();
        let created_medias = DATABASE.create::<Vec<Self>>("media").content(&self).await?;

        // Check if it was really created.
        if created_medias.is_empty() {
            warn!("No media was created.");
            dbg!(self);
        }

        info!("New media created.");
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
