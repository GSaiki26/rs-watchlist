// Libs
use axum::{http::StatusCode, Json};
use surrealdb::sql::{Id, Thing};
use tracing::{error, info, warn};

use super::response_body::ResponseBody;
use crate::models::media_model::Media;
use crate::models::model_trait::ModelTrait;
use crate::models::user_model::User;
use crate::models::watchlist_model::Watchlist;
use crate::security::is_valid_field;

// Types
pub type Response = (StatusCode, Json<ResponseBody>);

// Functions
// Media
/**
 * A method to get some media in the database using its id.
*/
pub async fn get_media_from_id(media_id: Id) -> Result<Media, Response> {
    // Try to get the media using his id.
    match Media::from_id(media_id).await {
        Err(e) => {
            error!("Couldn\'t get the media. {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the media. Please call the admin."),
            ))
        }
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            ResponseBody::error("media not found. Check the id and try again."),
        )),
        Ok(Some(media)) => Ok(media),
    }
}

// User
/**
 * A method to login an user. Uses its id and password.
*/
pub async fn login_user(
    user_auth: (String, Option<String>),
    is_username: bool,
) -> Result<User, Response> {
    info!("Trying to login the user...");

    // Define the default error_message.
    let response_error = (
        StatusCode::UNAUTHORIZED,
        ResponseBody::error("Check the credentials and try again."),
    );

    // Check the authorization.
    let (user_identifier, password) = match user_auth {
        (user_identifier, Some(pass)) => (user_identifier, pass),
        (_, None) => {
            info!("Password not included.");
            return Err(response_error);
        }
    };

    // Get the user from the database and check the password.
    let user_db = if is_username {
        get_user_from_username(&user_identifier).await?
    } else {
        get_user_from_id(Id::from(user_identifier)).await?
    };
    match user_db.is_login_valid(password) {
        false => Err(response_error),
        true => {
            info!("User successfully logged in.");
            Ok(user_db)
        }
    }
}

/**
 * A method to get some user in the database using his id.
*/
pub async fn get_user_from_id(user_id: Id) -> Result<User, Response> {
    // Try to get the user using his id.
    match User::from_id(user_id).await {
        Err(e) => {
            error!("Couldn\'t get the user. {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the user. Please call the admin."),
            ))
        }
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            ResponseBody::error("User not found. Check the id and try again."),
        )),
        Ok(Some(user)) => Ok(user),
    }
}

/**
 * A method to get some user in the database using his username.
*/
pub async fn get_user_from_username(username: &str) -> Result<User, Response> {
    // Check if the username is valid.
    if !is_valid_field(username, 20) {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseBody::error("The username is invalid. Check the parameters and try again."),
        ));
    }

    // try to get the user by his username.
    match User::from_username(username).await {
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponseBody::error("Couldn\'t get the user. Please call the admin."),
        )),
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            ResponseBody::error("User not found. Check the username and try again."),
        )),
        Ok(Some(user)) => Ok(user),
    }
}

// Watchlist
/**
 * A method to get some watchlist in the database using its id.
*/
pub async fn get_watchlist_from_id(watchlist_id: Id) -> Result<Watchlist, Response> {
    // Try to get the watchlist using his id.
    match Watchlist::from_id(watchlist_id).await {
        Err(e) => {
            error!("Couldn\'t get the watchlist. {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the watchlist. Please contact the admin."),
            ))
        }
        Ok(None) => {
            info!("watchlist not found.");
            Err((
                StatusCode::NOT_FOUND,
                ResponseBody::error("watchlist not found. Check the id and try again."),
            ))
        }
        Ok(Some(watchlist)) => {
            info!("watchlist found.");
            Ok(watchlist)
        }
    }
}

/**
 * A method to check if all members are valid.
*/
pub async fn are_members_valid(owner: &Thing, members: &Vec<String>) -> Result<(), Response> {
    info!("Checking if all members from the watchlist are valid.");
    if members.contains(&owner.id.to_string()) {
        warn!("The owner is a member.");
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseBody::error("Some member is valid. Check the parameters and try again."),
        ));
    }

    // Check if all members are valid.
    for member_id in members {
        match User::from_id(Id::from(member_id)).await {
            Err(e) => {
                error!("Couldn\'t check if all members are valid. {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBody::error(
                        "Couldn\'t create the watchlist. Please contact the admin.",
                    ),
                ));
            }
            Ok(None) => {
                info!("Some member is invalid.");
                return Err((
                    StatusCode::BAD_REQUEST,
                    ResponseBody::error(
                        "Some member is valid. Check the parameters and try again.",
                    ),
                ));
            }
            Ok(Some(_)) => (),
        }
    }

    info!("All members are valid.");
    Ok(())
}

/**
 * A method to get all the related watchlist to the user.
*/
pub async fn get_all_watchlist_from_user(logged_user: &User) -> Result<Vec<Watchlist>, Response> {
    info!(
        "Getting all the watchlists from {}.",
        logged_user.id.as_ref().unwrap()
    );

    // Get owned watchlists.
    let mut watchlists = match logged_user.get_watchlists_as_owner().await {
        Err(e) => {
            error!("Couldn\'t get the owned watchlists. {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the watchlists. Please contact the admin."),
            ));
        }
        Ok(watchlists) => watchlists,
    };

    // Get the watchlists as member.
    let mut member_watchlists = match logged_user.get_watchlists_as_member().await {
        Err(e) => {
            error!("Couldn\'t get the watchlists as member. {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the watchlists. Please contact the admin."),
            ));
        }
        Ok(member_watchlists) => member_watchlists,
    };

    watchlists.append(&mut member_watchlists);
    info!("The watchlists were successfully retrieved.");

    Ok(watchlists)
}
