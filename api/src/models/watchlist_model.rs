// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::{error, info, warn};

use super::{media_model::Media, model_trait::ModelTrait};
use crate::database::DATABASE;

// Structs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Watchlist {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Thing>,
    pub members: Vec<Thing>,
    pub title: String,
    pub description: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistRequest {
    pub members: Vec<String>,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistResponse {
    pub id: String,
    pub owner: String,
    pub members: Vec<String>,
    pub title: String,
    pub description: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// Implementations
impl Watchlist {
    /**
     * A method to get all the media from the watchlist.
     */
    pub async fn get_media(&self) -> surrealdb::Result<Vec<Media>> {
        info!("Getting media from {}.", self.id.as_ref().unwrap());
        match DATABASE
            .query("SELECT * FROM media WHERE watchlist = $watchlist_id")
            .bind(("watchlist_id", self.id.as_ref().unwrap()))
            .await?
            .take(0)
        {
            Err(e) => {
                error!(
                    "Couldn\'t get the media from {}.",
                    self.id.as_ref().unwrap()
                );
                Err(e)
            }
            Ok(medias) => {
                info!("The media were successfully retrieved.");
                Ok(medias)
            }
        }
    }

    /**
     * A method to check if the watchlist is owned by the given user.
     */
    pub fn is_owner(&self, owner: &Thing) -> bool {
        self.owner.as_ref().unwrap() == owner
    }

    /**
     * A method to check if the watchlist has the given member.
     */
    pub fn has_member(&self, member: &Thing) -> bool {
        self.members.contains(member)
    }

    /**
     * A method to convert the current watchlist to a WatchlistResponse
     */
    pub fn to_watchlist_response(&self) -> WatchlistResponse {
        WatchlistResponse::from(self.clone())
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
                    DEFINE FIELD members ON TABLE watchlist TYPE array;
                    DEFINE FIELD members.* ON TABLE watchlist TYPE record<user>;
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
        self.updated_at = Datetime::default();
        info!("Syncing {} in the database...", self.id.as_ref().unwrap());
        DATABASE
            .update::<Option<Self>>(("watchlist", self.id.clone().unwrap()))
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
        self.created_at = Datetime::default();
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

    fn merge(&mut self, value: Self) {
        // Merge the watchlist with another watchlist.
        self.members = value.members;
        self.title = value.title;
        self.description = value.description;
    }

    async fn delete(self) -> surrealdb::Result<()> {
        // Check if the watchlist has an id.
        if let Some(id) = self.id.clone() {
            info!("Deleting {}...", &id);
            DATABASE.delete::<Option<Watchlist>>(&id).await?;
            info!("The {} was deleted.", id);
        } else {
            warn!("The watchlist has no id.");
        }

        Ok(())
    }
}

impl From<WatchlistRequest> for Watchlist {
    fn from(value: WatchlistRequest) -> Self {
        // Convert the watchlist request to a watchlist.
        Self {
            id: None,
            owner: None,
            members: value
                .members
                .iter()
                .map(|member| Thing {
                    id: Id::from(member),
                    tb: String::from("user"),
                })
                .collect(),
            title: value.title,
            description: value.description,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        }
    }
}

impl From<Watchlist> for WatchlistResponse {
    fn from(value: Watchlist) -> Self {
        Self {
            id: value.id.expect("Logic error").id.to_string(),
            owner: value.owner.expect("Logic error").id.to_string(),
            members: value
                .members
                .iter()
                .map(|member| member.id.to_string())
                .collect(),
            title: value.title,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
