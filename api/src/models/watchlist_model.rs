// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::{info, warn};

use super::model_trait::ModelTrait;
use crate::database::DATABASE;

// Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct Watchlist {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub owner: Thing,
    pub members: Vec<Thing>,
    pub title: String,
    pub description: String,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

// Implementations
impl Watchlist {
    /**
     * A method to check if the watchlist is owned by the given user.
     */
    pub fn is_owner(&self, owner: &Thing) -> bool {
        self.owner == owner.clone()
    }

    /**
     * A method to check if the watchlist has the given member.
     */
    pub fn has_member(&self, member: &Thing) -> bool {
        self.members.contains(member)
    }
}

impl ModelTrait<Watchlist> for Watchlist {
    async fn from_id(id: Id) -> surrealdb::Result<Option<Self>> {
        // Create the thing.
        let thing = Thing {
            id,
            tb: String::from("watchlist"),
        };

        // Get the watchlist.
        info!("Getting {}.", &thing);
        match DATABASE.select::<Option<Self>>(thing.clone()).await? {
            None => {
                info!("No {} found.", &thing);
                Ok(None)
            }
            Some(watchlist) => {
                info!("{} found.", thing);
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
                    DEFINE FIELD owner ON TABLE watchlist TYPE record<user>;
                    DEFINE FIELD members ON TABLE watchlist TYPE array<record<user>>;
                    DEFINE FIELD title ON TABLE watchlist TYPE string ASSERT $value = /^[a-zA-Z0-9!@#$%&*_\\-+.,<>;\\/? ]{3,20}$/;
                    DEFINE FIELD description ON TABLE watchlist TYPE string ASSERT $value = /^[a-zA-Z0-9!@#$%&*_\\-+.,<>;\\/? ]{3,60}$/;
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
            return self.create().await;
        }

        // Sync the watchlist in the database.
        self.updated_at = Some(Datetime::default());
        info!("Syncing {} in the database...", self.id.as_ref().unwrap());
        DATABASE
            .update::<Vec<Self>>("watchlist")
            .content(&self)
            .await?;
        info!("Synced {} in the database.", self.id.as_ref().unwrap());

        Ok(())
    }

    async fn create(&mut self) -> surrealdb::Result<()> {
        // Generate a new thing for the watchlist.
        info!("Creating a new watchlist...");
        self.id = Some(Self::generate_new_ulid("watchlist").await?);

        // Create the watchlist in the database.
        self.created_at = Some(Datetime::default());
        self.updated_at = self.created_at.clone();
        let created_watchlists = DATABASE
            .create::<Vec<Self>>("watchlist")
            .content(&self)
            .await?;

        // Check if it was really created.
        if created_watchlists.is_empty() {
            warn!("No watchlist was created.");
            dbg!(&self);
        }

        info!("The new {} was created.", self.id.as_ref().unwrap());
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
