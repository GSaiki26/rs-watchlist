// Libs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use tracing::{info, warn};

use super::model_trait::ModelTrait;
use super::watchlist_model::Watchlist;
use crate::database::DATABASE;
use crate::security::get_sha512;

// Structs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,

    pub password: String,

    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
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
     * A method to get all watchlist the user OWNS.
     */
    pub async fn get_watchlists_as_owner(&self) -> surrealdb::Result<Vec<Watchlist>> {
        // Get the watchlists.
        info!("Getting all watchlists from {}.", self.id.as_ref().unwrap());
        let watchlists: Vec<Watchlist> = DATABASE
            .query("SELECT * FROM watchlist WHERE owner = $owner")
            .bind(("owner", self.id.as_ref().unwrap()))
            .await?
            .take(0)?;

        info!(
            "{} watchlists found for {}.",
            watchlists.len(),
            self.id.as_ref().unwrap()
        );

        Ok(watchlists)
    }

    /**
     * A method to get all watchlist the user is a member.
     */
    pub async fn get_watchlists_as_member(&self) -> surrealdb::Result<Vec<Watchlist>> {
        // Get the watchlists.
        info!(
            "Getting all member watchlists from {}.",
            self.id.as_ref().unwrap()
        );
        let watchlists: Vec<Watchlist> = DATABASE
            .query("SELECT * FROM watchlist WHERE members CONTAINS $member_id")
            .bind(("member_id", self.id.as_ref().unwrap()))
            .await?
            .take(0)?;

        info!(
            "{} watchlists found for {}.",
            watchlists.len(),
            self.id.as_ref().unwrap()
        );

        Ok(watchlists)
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
     * A method to convert the current User to a UserResponse
     */
    pub fn to_user_response(&self) -> UserResponse {
        let user = self.clone();
        UserResponse::from(user)
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
                    DEFINE FIELD username ON TABLE user TYPE string VALUE string::lowercase($value) ASSERT $value = /^[a-z0-9!@#$%&*_\\-+.,<>;\\/? ]{3,20}$/;
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

        // Sync the user in the database.
        self.updated_at = Datetime::default();
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
        self.created_at = Datetime::default();
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

    fn merge(&mut self, value: Self) {
        // Merge the user with another user.
        self.username = value.username;
        self.password = value.password;
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

impl From<UserRequest> for User {
    fn from(mut value: UserRequest) -> Self {
        // Treat the username and password.
        value.username = value.username.to_lowercase();
        value.password = get_sha512(value.password.as_bytes());

        // Create the user.
        User {
            id: None,
            username: value.username,
            password: value.password,
            created_at: Datetime::default(),
            updated_at: Datetime::default(),
        }
    }
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id.expect("Logic error.").id.to_string(),
            username: value.username,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
